use std::io; //to obtain user input and print the result as output

fn main() {
    println!("Guess the number.");

    println!("Please input your guess:");

    let mut guess= String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");
}
