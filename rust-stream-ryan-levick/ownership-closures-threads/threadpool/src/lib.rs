/**
 *
 * These are OS threads. Green threads were removed
 * from Rust, because in order to have them you need
 * to have a runtime, and that adds overhead that goes
 * against the Rust philosophy of zero cost abstractions.
 *
 * Tokio tasks are green threads.
 *
 */
pub struct ThreadPool {
	_handles: Vec<std::thread::JoinHandle<()>>
}

impl ThreadPool {
	pub fn new(num_threads: u8) -> Self {
		let _handles = (0..num_threads)
			.map(|_| std::thread::spawn(|| {}))
			.collect();
		Self {
			_handles
		}
	}

	// needs & here, otherwise the first call
	// to execute would move the ownership
	pub fn execute<T: Fn()>(&self, work: T) {}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
		let pool = ThreadPool::new(10);
		pool.execute(|| println!("Hello from thread"));
		pool.execute(|| println!("Hello from thread"));
    }
}
