fn clone_and_modify(s: &String) -> String {
    let mut cloned_string = s.clone(); // Your code here
    cloned_string.push_str("World!");
    cloned_string
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}