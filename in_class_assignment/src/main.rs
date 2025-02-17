fn borrow_to_mut_watchout() {
    let mut word = "UT".to_string(); 
    fn update(word: &mut String) {
        word.push_str("RGV");
    }
    update(&mut word);
    println!("{word}")
}

fn main(){
borrow_to_mut_watchout()
}