enum Insurance {
 Car(string),
 House(u16),

}

impl Insurance{
    fn show_info(&self){
        match self{
            Insurance::Car(model) => println! ("My car model is {:?}", model),
            Insurance::House(year) => println! ("My house was built in {}", year),
        }
    }
}

fn main() {
let car = Insurance::Car("BMW".to_string());

}