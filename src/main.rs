mod bindable_thread_pool;
use bindable_thread_pool::POLICY;
use bindable_thread_pool::*;
fn main() {
    BindableThreadPool::new(POLICY::ROUND_ROBIN_CORE)
        .num_threads(2)
        .build_global()
        .expect("Thread pool build failed");
    println!("Hello, world!");
}
