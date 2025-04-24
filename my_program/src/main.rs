fn last_lecture_problem_fixing(){
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { 
        let mut largest = list[0]; // we need Copy trait to achieve that operation
        
        for &item in list.iter() {
            if item > largest { // we need PartialOrd trait to be able to compare elements
                largest = item;
            }
        }
        largest
    }

    // where clause
    fn largest_2<T>(list: &[T]) -> T 
        where T: PartialOrd + Copy,
        {
        let mut largest = list[0]; // we need Copy trait to achieve that operation
        
        for &item in list.iter() {
            if item > largest { // we need PartialOrd trait to be able to compare elements
                largest = item;
            }
        }
        largest
    }


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_2(&char_list);
    println!("The largest char is {}", result);
}
fn main(){
   last_lecture_problem_fixing();
}