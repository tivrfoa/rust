pub struct ThreadPool;

impl ThreadPool {
	pub fn new() -> Self {
		Self
	}

	// needs & here, otherwise the first call
	// to execute would move the ownership
	pub fn execute(&self) {}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
		let pool = ThreadPool::new();
		pool.execute();
    }
}
