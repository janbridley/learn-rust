use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn _enter_guess() -> String {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Conversion to i32 must be done in main to avoid continue outside loop
    return guess
}

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);


    println!("The secret number is {secret_number}");

    loop {

        // parse returns a Result (enum with vars Ok and Err)
        // match is like a switch (?) that returns num or continues
        let guess: i32 = match _enter_guess().trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Invalid number!"); continue},
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {println!("You win!"); break;},
        }
    }

}
