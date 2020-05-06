use std::sync::mpsc::channel;
use std::sync::Mutex;

/**
 * He uses: VSCode with Rust Analyzer
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

		// channel requires a concrete type, not a trait.
		// the reason for that is it needs to know the thing
		// it's receiving for.
		// trait has no size.
		// trait is just a compile-time concept
		// Structs actually have a layout memory. They have a
		// defined size so the compiler knows how much space
		// to reserve.
		//
		// When you don't care or even know what concrete type
		// they are, the best thing to do is to take that trait
		// and box it, and put it on the heap so that we can call
		// the trait methods in a way that is called dynamic
		// dispatch.
		//
		// A Box is a heap allocated pointer.
		//
		// The closure traits are not ordinary traits. The
		// parentheses are needed in order to declare its
		// arguments.
		//
		let (sender, receiver) = channel::<Box<dyn Fn()>>();
		let receiver = Mutex::new(receiver);
		let mut _handles = vec![];
		
		for _ in 0..num_threads {
			let handle = std::thread::spawn(|| {
				loop {
					// check for work
					let work = receiver.lock().unwrap().recv().unwrap();
					work();
				}
			});
			_handles.push(handle);
		}
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


/**
 * Must pass some type T that implements the Debug trait.
 *
 * When you have generic argument, you'll literally create
 * copies of that function for every single type. The
 * compiler will specializee for each type.
 *
 * When the argument is a reference to trait, it will go
 * through a virtual call table (v-table) to call functions
 * on that. It's a little bit slower, because you have to
 * go through a v-table to call it, but there's only one
 * foo function, so you save space.
 *
 *
 */
fn foo1<T: std::fmt::Debug>(item: T) {
	println!("Foo");
}

/**
 * This works because it is receiving a reference (&)
 * and Rust knows the size of a pointer.
 *
 * As the parameter is a pointer, it means it will go
 * through a v-table.
 */
fn foo2(item: &dyn std::fmt::Debug) {
	println!("Foo");
}

fn bar1() {
	foo1(1u8);
	foo1(String::new());
}

fn bar2() {
	foo2(&1u8);
	foo2(&String::new());
}

