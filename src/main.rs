mod bindable_thread_pool;
use bindable_thread_pool::POLICY;
use bindable_thread_pool::*;
pub fn fibonacci_recursive(n: i32) -> u64 {
	if n < 0 {
		panic!("{} is negative!", n);
	}
	match n {
		0     => panic!("zero is not a right argument to fibonacci_reccursive()!"),
		1 | 2 => 1,
		3     => 2,
		/*
		50    => 12586269025,
		*/
		_     => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
	}
}
fn main() {
let mypool =
    BindableThreadPool::new(POLICY::ROUND_ROBIN_NUMA)
        .num_threads(16)
        .build()
        .expect("Thread pool build failed");
	mypool.install(|| fibonacci_recursive(50));
    println!("Hello, world!");
}
