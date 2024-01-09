use std::io; 
use rand::Rng;

fn main() {
    println!("Let's get start number guessing game"); 

    let secret_number = rand::thread_rng().gen_range(1..=100);

    print("The secret number is: {secret_number}");

    println!("Please input your guess: ");

    let mut guess = String::new(); 

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); 

    println!("you guessed: {guess}");

}