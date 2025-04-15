//Task 3: Vector Transformation

// 2. using a for loop

fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
F: Fn(i32) -> i32,
{
let mut result = Vec::new();
for x in vec {
result.push(f(x)); // Apply the closure
}
result// Your implementation here
}

fn main() {
let numbers = vec![1, 2, 3];

let doubled = process_vector(numbers.clone(), |x| {
  x*2  // Implement: multiply each number by 2
});

    let replaced = process_vector(numbers, |x| {
if x > 2 {0} else {x} // Implement: if number > 2, replace with 0, else keep number
});

println!("Doubled: {:?}", doubled);
println!("Replaced: {:?}", replaced);
}