use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess The Number");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => print!("{}", "Too Small!".red()),
            Ordering::Greater => print!("{}", "Too Big!!".red()),
            Ordering::Equal => {
                print!("{}", "You Win!!".green());
                break;
            }
        }
    }
}
