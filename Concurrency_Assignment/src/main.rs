//ASSIGNMENT 1: Spawning Threads and Joining Them

// use std::thread;
// use std::time::Duration;

// fn main() {
//     println!("Main thread starting");
    
//     // TODO: Create a vector to store thread handles
//     let mut handles = vec![];
    
//     // TODO: Spawn 3 threads
//     for i in 1..=3 {
//         // TODO: Spawn a thread and store its handle
//         let handle = thread::spawn(move || {
//             // Simulate some work
//             println!("Thread {} starting", i);
//             thread::sleep(Duration::from_millis(500));
//             println!("Thread {} finished", i);
//         });
        
//         // TODO: Store the handle
//         handles.push(handle);
//     }
    
//     // TODO: Wait for all threads to complete
//     for handle in handles{
//         handle.join().unwrap();
//     }
    
//     println!("All threads completed.");
// }

//--------------------------------------------------------//

// ASSIGNMENT 2: Sharing Counter Data Between Threads
// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     // Create a shared counter using Arc and Mutex
//     let counter = Arc::new(Mutex::new(0));
    
//     // Create a vector to store thread handles
//     let mut handles = vec![];
    
//     // Spawn 5 threads
//     for _i in 1..=5 {
//         // Clone the Arc for the thread
//         let counter_clone = Arc::clone(&counter);
        
//         // Spawn a thread that increments the counter 10 times
//         let handle = thread::spawn(move || {
//             for _ in 0..10 {
//                 let mut num = counter_clone.lock().unwrap();
//                 *num += 1;
//                 // Optional: Uncomment to see which thread is incrementing
//                 // println!("Thread {} incremented counter to {}", i, *num);
//             }
//         });
        
//         handles.push(handle);
//     }
    
//     // Wait for all threads to complete
//     for handle in handles {
//         handle.join().unwrap();
//     }
    
//     // Print the final value of the counter
//     println!("Final counter value: {}", *counter.lock().unwrap());
// }

//--------------------------------------------------------//

