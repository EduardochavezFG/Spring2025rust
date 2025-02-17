fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 42; // Hard-coded secret number
    let mut attempts = 0;
    let  guesses = [50, 30, 42]; // Simulated user input
    let mut index = 0;
    
    loop {
        let guess = guesses[index];
        index += 1;
        
        attempts += 1;
        
        match check_guess(guess, secret) {
            0 => {
                println!("Correct! You guessed the number in {} attempts.", attempts);
                break;
            }
            1 => println!("Too high! Try again."),
            -1 => println!("Too low! Try again."),
            _ => unreachable!(),
        }
    }
}
