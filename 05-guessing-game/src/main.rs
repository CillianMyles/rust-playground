fn main() {
    println!("Hello...");
    println!("Guess a number: ");

    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {guess}");
}
