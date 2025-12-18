use rand::Rng; // Random number generator
use std::cmp::Ordering;
use std::io; //to obtain user input and print the result as output // The Ordering type is an enum and has the variants Less, Greater, and Equal

fn main() {
    println!("Guess the number.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //  local to the current thread of execution and is seeded by the operating system.

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Correct!");
                break
            }
            Ordering::Less => println!("Way behind"),
            Ordering::Greater => println!("Way ahead"),
        }
    }
    println!("The secret number is: {secret_number}");

}
