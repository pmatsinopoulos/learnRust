use std::io;

fn main() {
    println!("Guess the number");
    println!("Please, input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guessed number");

    println!("You guessed: {}", guess);

    let correct_guess = guess.contains("5");

    println!("Correct Guess? {}", correct_guess);
}
