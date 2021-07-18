use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret = rand::thread_rng().gen_range(1..101);
    loop {
    println!("Please input your Guess");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // You can fail the program on ERR while parsing with .expect() method
    // let guess : u32 = guess.trim().parse().expect("Please type a number!");

    // Its better to handle the error with match and using Ok and Err as arms
    let guess : u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("[ERR] invalid input please type a number");
            continue;
        }
    };
    


    println!("Your guessed {}", guess);
    match guess.cmp(&secret){
        Ordering::Less => print!("\tbut its too smol"),
        Ordering::Greater => print!("\tbut its too big"),
        Ordering::Equal => {
            print!("\tThat is right you win!\n");
            break;
        }
    }

    }
}
