use std::io;


fn main() {
    println!("Guess the Number!");

    println!("Please input your Guess");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("Your guessed {}", guess)
}
