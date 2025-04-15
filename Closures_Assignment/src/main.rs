//Task 1: Basic Closure

//fn main() {
//let operation = |a: i32, b: i32| {
//  a*b
//};

// println!("Result: {}", operation(10, 5));
//}

// ------------------------------------------------------//
//Task 2: Enviorment Capture

//fn track_changes() {
//    let mut tracker = 0;
//    let mut update = || {
//    tracker += 1;
//    println!("Increment:{}",tracker)
//    };

//    update();
//    update();
//}

//fn main() {
//    track_changes();
//}


// ------------------------------------------------------//
//Task 3: Vector Transformation

// 1. using map and collect 

// fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
// F: Fn(i32) -> i32,
// {
 // vec.into_iter().map(f).collect()// Your implementation here
// }

// fn main() {
// let numbers = vec![1, 2, 3];

// let doubled = process_vector(numbers.clone(), |x| {
//    x*2 // Implement: multiply each number by 2
// });

// let replaced = process_vector(numbers, |x| {
//    if x > 2 {0} else {x}// Implement: if number > 2, replace with 0, else keep number
// });

// println!("Doubled: {:?}", doubled);
// println!("Replaced: {:?}", replaced);
// }

//-----------------------------------------------------------//
// 2. using a for loop

// fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
// F: Fn(i32) -> i32,
// {
// let mut result = Vec::new();
// for x in vec {
// result.push(f(x)); // Apply the closure
// }
// result// Your implementation here
// }

// fn main() {
// let numbers = vec![1, 2, 3];

// let doubled = process_vector(numbers.clone(), |x| {
//   x*2  // Implement: multiply each number by 2
// });

//     let replaced = process_vector(numbers, |x| {
// if x > 2 {0} else {x} // Implement: if number > 2, replace with 0, else keep number
// });

// println!("Doubled: {:?}", doubled);
// println!("Replaced: {:?}", replaced);
// }

//-----------------------------------------------------------//
// //Task 5: Lazy Computation
// use std::{thread, time::Duration};

// struct ComputeCache<T>
// where
//     T: Fn() -> String,
// {
//     computation: T,
//     result: Option<String>,
// }

// impl<T> ComputeCache<T>
// where
//     T: Fn() -> String,
// {
//     fn new(computation: T) -> Self {
//         ComputeCache {
//             computation,
//             result: None,
//         }
//     }

//     fn get_result(&mut self) -> String {
//         match &self.result {
//             Some(value) => {
//                 println!("Retrieved from cache instantly!");
//                 value.clone()
//             }
//             None => {
//                 let value = (self.computation)();
//                 self.result = Some(value.clone());
//                 value
//             }
//         }
//     }
// }

// fn main() {
//     let mut cache = ComputeCache::new(|| {
//         println!("Computing (this will take 2 seconds)...");
//         thread::sleep(Duration::from_secs(2));
//         "Hello, world!".to_string()
//     });

//     println!("First call:");
//     println!("Result: {}", cache.get_result());

//     println!("\nSecond call:");
//     println!("Result (cached): {}", cache.get_result());
// }