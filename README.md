## WHAT
This crate allows you to create a bindable thread pool. This is simply a wrapper around the Rayon's thread pool. It supports all the "major" functions with the same signature. At the time of creation of this pool, a binding policy is specified. All the threads in this pool will be bound to the cores using this policy. Currently we have the following two policies:

* ROUND_ROBIN_NUMA : In case you are working with a NUMA machine, you can choose to assign threads to cores in the NUMA machine in a round robin fashion. This implies that if you have 3 NUMA nodes with 4 cores each, and you try to bind:
    1. 2 Threads - Core 0 of NUMA node #0 and Core 0 of NUMA node #1 have get thread each.
    2. 4 Threads - Cores 0 and 1 of NUMA node #0 and Cores 0 and 1 of NUMA node #1 all have one thread each.
    3. 5 Threads - 4 Threads get mapped as above and the 5th one ends up on Core 2 of NUMA node #0.

* ROUND_ROBIN_CORE : This is the simpler case where the first NUMA node gets filled up first and then assignment takes place on subsequent NUMA nodes.


Note that in all the above cases, a core is a physical core, and not a logical core. This means that if the machine is hyperthreaded, the system will never map two threads on the same physical core. In fact, it singlifies the bitmap, which means that the OS won't even migrate a thread across these two logical cores (that map to the same physical core).

## WHY
This crate uses an existing HWLOC-RS crate by daschl. However, it is much more programmer friendly since the same API is exposed and that makes it super easy to use.

## HOW
1. Just clone and add the path (pointing to where you cloned) in the Cargo.toml. Name of the package is thread_binder.
2. Create the thread pool using BindableThreadPool::new(POLICY::XXX) where XXX is one of the above policies. It is necessary to give some policy for binding. Rest of the usage is same as the Rayon thread pool.
