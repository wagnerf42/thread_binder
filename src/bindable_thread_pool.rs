extern crate hwloc;
extern crate libc;
extern crate rayon;
use self::hwloc::{CpuSet, ObjectType, Topology, CPUBIND_THREAD};
use self::rayon::{ThreadPool, ThreadPoolBuildError, ThreadPoolBuilder};
use std::sync::{Arc, Mutex};

pub struct BindableThreadPool {
    builder: ThreadPoolBuilder,
    bind_policy: POLICY,
    num_threads: usize,
}
/// This enum specifies whether you want to pack the threads on one NUMA node or assign them on
/// multiple NUMA nodes in a round robin fashion.
pub enum POLICY {
    /// Threads get assigned to the first available CPU in a NUMA node in a round robin fashion.
    ROUND_ROBIN_NUMA,
    /// Threads get assigned to the first available PU in a CPU in a round robin fashion.
    ROUND_ROBIN_CORE,
    /// Threads get assigned to the first available PU.
    ROUND_ROBIN_PU,
}

fn cpuset_for_core(topology: &Topology, idx: usize) -> CpuSet {
    let cores = (*topology).objects_with_type(&ObjectType::Core).unwrap();
    let numa_nodes = (*topology)
        .objects_with_type(&ObjectType::NUMANode)
        .unwrap_or(Vec::new());
    match cores.get(idx) {
        Some(val) => val.cpuset().unwrap(),
        None => panic!(
            "I won't allow you to have {} more threads than logical cores!",
            idx - cores.len() + 1
        ),
    }
}

fn get_thread_id() -> libc::pthread_t {
    unsafe { libc::pthread_self() }
}

impl BindableThreadPool {
    /// Creates a new LoggedPoolBuilder.
    pub fn new(bind_policy: POLICY) -> Self {
        BindableThreadPool {
            builder: ThreadPoolBuilder::new(),
            bind_policy,
            num_threads: 0,
        }
    }

    /// Set number of threads wanted.
    pub fn num_threads(self, num_threads: usize) -> Self {
        BindableThreadPool {
            builder: self.builder.num_threads(num_threads),
            bind_policy: self.bind_policy,
            num_threads,
        }
    }

    /// Build the `ThreadPool`.
    pub fn build(self) -> Result<ThreadPool, ThreadPoolBuildError> {
        let topo = Mutex::new(Topology::new());
        //bind_main_thread(&topo);
        let pool = match self.bind_policy {
            POLICY::ROUND_ROBIN_NUMA => self
                .builder
                .start_handler(move |thread_id| {
                    bind_numa(thread_id, &topo);
                }).build(),
            _ => self
                .builder
                .start_handler(move |thread_id| {
                    binder(thread_id, &topo);
                }).build(),
        };
        pool
    }

    pub fn build_global(self) -> Result<(), ThreadPoolBuildError> {
        let topo = Mutex::new(Topology::new());
        //bind_main_thread(&topo);
        match self.bind_policy {
            POLICY::ROUND_ROBIN_NUMA => self
                .builder
                .start_handler(move |thread_id| {
                    bind_numa(thread_id, &topo);
                }).build_global(),
            _ => self
                .builder
                .start_handler(move |thread_id| {
                    binder(thread_id, &topo);
                }).build_global(),
        }
    }
}

fn bind_main_thread(topo: &Mutex<Topology>) {
    let pthread_id = get_thread_id();
    let mut locked_topo = topo.lock().unwrap();
    let mut bind_to = cpuset_for_core(&locked_topo, 0);
    bind_to.singlify();
    println!("binding {} to {}", pthread_id, bind_to);
    locked_topo
        .set_cpubind_for_thread(pthread_id, bind_to, CPUBIND_THREAD)
        .unwrap();
    println!("binding done");
    let after = locked_topo.get_cpubind_for_thread(pthread_id, CPUBIND_THREAD);
    println!("Thread {}, bind to {:?}", 0, after);
}

fn bind_numa(thread_id: usize, topo: &Mutex<Topology>) {
    let pthread_id = get_thread_id();
    let mut locked_topo = topo.lock().unwrap();
    let num_numa_nodes = (locked_topo)
        .objects_with_type(&ObjectType::NUMANode)
        .unwrap_or(Vec::new())
        .len();
    let my_numa_node_index = thread_id % num_numa_nodes;
    let my_core_index = thread_id / num_numa_nodes;
    let mut my_core = {
        let cpu_list = locked_topo.objects_with_type(&ObjectType::Core).unwrap();
        let cpu_depth = cpu_list[0].depth();
        println!("CPU depth is {}", cpu_depth);
        let cpu_list = locked_topo.objects_at_depth(cpu_depth);
        cpu_list
            .get(my_numa_node_index * num_numa_nodes + my_core_index)
            .unwrap()
            .cpuset()
            .unwrap()
    };
    println!("want to bind to {:?}", my_core);
    my_core.singlify(); //This would give you "some" cpu node but you don't know which one.
    locked_topo
        .set_cpubind_for_thread(pthread_id, my_core, CPUBIND_THREAD)
        .unwrap();
    let after = locked_topo.get_cpubind_for_thread(pthread_id, CPUBIND_THREAD);
    println!("Thread {}, bind to {:?}", thread_id, after);
}
fn binder(thread_id: usize, topo: &Mutex<Topology>) {
    let pthread_id = get_thread_id();
    let mut locked_topo = topo.lock().unwrap();
    let mut bind_to = cpuset_for_core(&locked_topo, thread_id);
    bind_to.singlify();
    println!("binding {} to {}", pthread_id, bind_to);
    locked_topo
        .set_cpubind_for_thread(pthread_id, bind_to, CPUBIND_THREAD)
        .unwrap();
    println!("binding done");
    let after = locked_topo.get_cpubind_for_thread(pthread_id, CPUBIND_THREAD);
    println!("Thread {}, bind to {:?}", thread_id, after);
}
