🧠 Step-by-Step Human Explanation
use std::io;

We’re telling Rust:

“Hey, I want to use the input/output functions from the standard library,”
like reading user input from the keyboard.

use rand::Rng;

This brings in the random number generator trait — which gives us methods like .random_range() to generate random numbers.

use std::cmp::Ordering;

We’ll compare numbers later (your guess vs secret number).
Ordering helps handle comparison results as three possible outcomes:

Less

Greater

Equal

use colored::*;

This allows colored text in the terminal ✨
You can use .red(), .green(), etc., to make your output visually cool.

fn main() { ... }

This is where the program starts executing — the main() function is the entry point.

println!("Guess The Number");

Displays a welcome message on screen.
Rust’s println! macro automatically adds a newline at the end.

let secret_number = rand::rng().random_range(1..=100);

rand::rng() → gives a random number generator.

.random_range(1..=100) → generates a random number between 1 and 100 (inclusive).
That’s the number the player has to guess!

loop { ... }

Creates an infinite loop — the game keeps asking for guesses until the player gets it right.
We’ll break out of it when the user wins.

let mut guess = String::new();

Creates an empty string variable that can be changed (mut = mutable).
We’ll store the user’s input in this.

io::stdin().read_line(&mut guess).expect("Failed to read line");

io::stdin() → takes input from keyboard.

.read_line(&mut guess) → puts that input into the guess variable.

.expect("Failed to read line") → if something goes wrong, it shows this error instead of crashing silently.

let guess: u32 = match guess.trim().parse() { ... };

We’re converting the input string into a number.

.trim() → removes spaces/newlines.

.parse() → tries to convert text to a number.

match → handles two possible outcomes:

Ok(num) → successful conversion (store it in guess).

Err(_) → failed (user didn’t type a number), so continue restarts the loop and asks again.

This prevents the game from crashing if someone types “hello”.

println!("You guessed: {}", guess);

Just shows what the player typed — for confirmation.

match guess.cmp(&secret_number) { ... }

Now the fun part: comparing the player’s guess to the secret number.

.cmp(&secret_number) → compares two numbers and returns:

Ordering::Less → if guess < secret

Ordering::Greater → if guess > secret

Ordering::Equal → if guess == secret

Then we print colored messages accordingly.

Ordering::Less => print!("{}", "Too Small!".red()),

If the guess is smaller, print “Too Small!” in red.

Ordering::Greater => print!("{}", "Too Big!!".red()),

If it’s too large, print “Too Big!!” in red.

Ordering::Equal => { ... }

If the guess matches the secret number:

Print “You Win!!” in green 🎉

break; exits the loop — ending the game.

🧩 How Cargo.toml should look

Make sure you have these dependencies:

[dependencies]
rand = "0.9"
colored = "2"


Then run:

cargo build
cargo run

🏁 Output Example
Guess The Number
Please input your guess:
50
You guessed: 50
Too Small!
Please input your guess:
75
You guessed: 75
You Win!!


Would you like me to show you how to add attempt counting (like “You guessed it in 5 tries!”) next?