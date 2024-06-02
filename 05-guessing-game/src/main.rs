use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(0..=100);
    println!("Secret number: {secret_number}");

    println!("Guess a number: ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let guess: u32 = match guess.trim().parse() {
        Ok(value) => {
            println!("You guessed: {guess}");
            value
        }
        Err(_) => panic!("Please input a number!"),
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("You win!"),
    }
}
