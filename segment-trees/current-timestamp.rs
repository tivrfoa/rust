use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn main() {
   let now = SystemTime::now();
   println!("{:?}", now);

   for _ in 0..3 {
	   let now = SystemTime::now();
	   let nano = now.elapsed().unwrap().subsec_nanos();
	   println!("{:?} {:?} {:?}", nano, nano%10, ((nano%10) as f32)/10.0);
	   let mut nano = nano as f32;
	   // let nano = nano / 500.0;
	   
	   let mut r: f32 = 0.0;
	   while nano > 0.0 {
		   r += nano % 10.0;
		   nano /= 10.0;
	   }
	   println!("{:?} {:?}", nano, r);
   }

   // we sleep for 2 seconds
   sleep(Duration::new(2, 0));
   match now.elapsed() {
       Ok(elapsed) => {
           // it prints '2'
           println!("{}", elapsed.as_secs());
       }
       Err(e) => {
           // an error occurred!
           println!("Error: {:?}", e);
       }
   }
}
