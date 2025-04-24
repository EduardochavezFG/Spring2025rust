fn capture_ownership_modify() {
    let nums = vec![1, 2, 3, 4, 5].into_iter();
    let product_through_iterator: Box<dyn FnOnce() -> i32> = Box::new(move || nums.product());
    let result: i32 = product_through_iterator();
    println!("{}", result);  // Output: 120
}
fn main() {
  capture_ownership_modify()
}
