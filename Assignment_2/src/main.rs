fn is_even(n: i32)-> bool{
    n % 2 == 0
    }
    
    fn main(){
        let numbers: [i32; 10] = [3,12,15,17,22,23,27,30,42, 25];
        
        for &num in numbers.iter()
        {
            if num % 3 == 0 && num % 5 == 0 {
                println!("{} would be FizzBuzz", num);
            }
            else if num % 3 == 0 {
                println!("{} would be Fizz", num);
            }
            else if num % 5 == 0 {
                println!("{} would be Buzz", num );
            }
            
            else{
                let even_or_odd = if is_even(num) {"Even"} else {"ODD"};
                println!("{}:{} ", num, even_or_odd);
                    
                }
            
            
        }
        
        let mut sum = 0;
        let mut i = 0;
        
        while i< numbers.len()
        {
            sum += numbers[i];
            i+= 1;
        }
        
        println!("the sum of all the numbers would be {}", sum);
        
        let mut max_number = numbers[0];
        for &num in numbers.iter(){
            if num > max_number{
                max_number = num;
            }
        }
        println!("the max number is  {}", max_number);
    }