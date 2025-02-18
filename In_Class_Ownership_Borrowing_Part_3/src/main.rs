#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
  *total = 0;  // Write your code here!
  for i in low..=high{
    *total+= i;
  } 
}

fn main() {
   let mut total = 0; // create necessary variables and test your function for low 0 high 100
   sum(&mut total, 0 ,100); 
   println!("Total sum: {}",total)// total should be 5050
}