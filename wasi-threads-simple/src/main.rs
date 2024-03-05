use std::thread;
use std::time::Duration;

fn main() {
   thread::spawn(|| {
       for i in 1..5 {
           println!("Hi number {} from the spawned thread!", i);
           thread::sleep(Duration::from_secs(3));
       }
   });

   for j in 1..10 {
       println!("Hi number {} from the main thread!", j);
       thread::sleep(Duration::from_secs(2));
   }
}
