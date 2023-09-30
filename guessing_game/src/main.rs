use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0..=100);
    
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // mut keyword is for mutable variable.
        // all variables in rust are immutable, adding the mut keyword allows for mutable variables.
        // ref: https://doc.rust-lang.org/std/keyword.mut.html
        let mut guess = String::new();

        //The & indicates that this argument is a reference, which gives you a way to let 
        // multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. 
        io::stdin()
            .read_line(&mut guess)
            // read_line returns Result, which is a enumeration value. 
            // Results varients include Ok and Err. 
            .expect("Failed to read line");
            // All Result return types need to hand the Err varient.
            // Rust will give a wwarning when a Result is unused.
        
        // Guess reassigned to u32 (unsigned 32-bit number)    
        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        // match keyword is for flow control and is much similar to the C++, JS and Java switch statement.

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
