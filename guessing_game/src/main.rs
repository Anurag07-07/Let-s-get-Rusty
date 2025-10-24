// This line brings in the standard Input/Output library, which we need to read player input from the keyboard.
use std::io;
// This line brings in the 'Rng' trait from the 'rand' library, allowing us to generate random numbers.
use rand::Rng;
// This line brings in the 'Ordering' enum, which is used to compare the player's guess to the secret number.
use std::cmp::Ordering;
// This line brings in the 'colored' library, allowing us to display text in different colors in the console.
use colored::*;

// This is the main function where the program execution always starts.
fn main() {
    // This prints a welcoming title message to the screen.
    println!("Guess The Number");

    // This line creates a random number generator and uses it to select a secret number
    // between 1 and 100 (inclusive). This number is the target for the player.
    let secret_number = rand::rng().random_range(1..=100);

    // This starts an infinite loop, meaning the game will continuously ask for guesses
    // until we explicitly tell it to stop (by using the 'break' command).
    loop {
        // This prompts the player to enter their next guess.
        println!("Please input your guess:");

        // This creates a new, empty, and *mutable* (changeable) variable to store the player's raw text input.
        let mut guess = String::new();

        // This block handles reading the line from the player:
        io::stdin()
            // It reads the line and puts the text into the 'guess' variable.
            .read_line(&mut guess)
            // If the input reading fails for some reason, the program will crash and show this message.
            .expect("Failed to read line");

        // This section attempts to convert the text input (String) into a real whole number (u32).
        // It uses a 'match' statement to handle two possible results:
        let guess: u32 = match guess.trim().parse() {
            // If the conversion is successful (Ok), we use the resulting number ('num').
            Ok(num) => num,
            // If the conversion fails (Err, meaning the user typed non-numbers), we ignore the error
            Err(_) => {
                // and skip immediately to the next turn of the loop, asking for a new input.
                continue;
            }
        };

        // This line confirms to the player what number they just entered.
        println!("You guessed: {}", guess);

        // This compares the player's number ('guess') with the hidden 'secret_number'
        // and executes code based on the 'Ordering' result.
        match guess.cmp(&secret_number) {
            // If the guess is smaller, it prints "Too Small!" in red color.
            Ordering::Less => print!("{}", "Too Small!".red()),
            // If the guess is larger, it prints "Too Big!!" in red color.
            Ordering::Greater => print!("{}", "Too Big!!".red()),
            // If the guess is equal, the player wins!
            Ordering::Equal => {
                // It prints the victory message in green color.
                print!("{}", "You Win!!".green());
                // This breaks out of the 'loop', which ends the game.
                break;
            }
        }
    }
}