//! Rayon thread pools with threads bound to single numa nodes.
mod bindable_thread_pool;
pub use bindable_thread_pool::{Policy, ThreadPoolBuilder};
