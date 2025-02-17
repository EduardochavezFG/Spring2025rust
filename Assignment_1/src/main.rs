const FREEZE:f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
  let t = f;
   (t-FREEZE) * 5.0/ 9.0 
    }
    fn  celsius_to_fahrenheit(c: f64) -> f64{
    let t = c; 
  (t * 9.0/5.0) + FREEZE 
    }


fn main() { 
 let  x = 32.0;
  println!("Temperature is {} C", fahrenheit_to_celsius(x));
  println!( "The Temperature is {} F ", celsius_to_fahrenheit(x));

  for i  in  1..5{
    let new_temp_f = x + i  as f64;
    println!("Temperature is {} and {}", new_temp_f, fahrenheit_to_celsius(new_temp_f))
  }

  }