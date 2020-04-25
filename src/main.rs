// Rust likes snake_case crate names
// ferris-says and Guessing Game from rust-lang
// ferris-says https://www.rust-lang.org/learn/get-started
// Guessing Game https://doc.rust-lang.org/1.8.0/book/guessing-game.html

extern crate rand; // Also does the equivalent of [use rand];

use ferris_says::say; // Import the say function from the ferrist_says "crate"
use std::io::{stdout, BufWriter};

use std::io; // Guessing Game //io library from the standard library
use std::cmp::Ordering;
use rand::Rng; // Allows rand::Rng to be in scope for the get_range method


fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap(); // Probably on a different thread, always appears last

    // Guessing Game
    println!();

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // 1 <= random_number < 101
    // (6*)

    //println!("The secret number is: {}", secret_number);

    loop { // Infinite loop! Will quit if parse() panic! s
        println!("Please input your guess.");

        let mut guess = String::new(); // (1*) (5*)

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line."); // If this is left off, Rust will give us a warning
        // (2*) (3*)

        let guess: u32 = match guess.trim().parse() { // match like cmp
            Ok(num) => num, // Moving from crashing to handling the error
            Err(_) => continue, // A 'Result,' and enum, is returned by parse().
            // num is the name of our successfully returned (and unwrapped) number
            // _ is a catch all error, continue is just like C
        };
        // .expect("Please type a number!");
        // (7*)

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){ // When just [&secret_number], we get a mismatched type error, need to convert
            // Rust has a string, static type system 
            // Rust also has type inference
            Ordering::Less => println!("Too small!"), // Ordering is an enum
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop and thus the program
            },
            // Matches 
        }
    }
    // (4*)


    // Random Notes

    // (1*)
        // Bindings in Rust are immutable by default
        //let foo = 5; // Immutable
        //let mut bar = 5; // Mutable
    
        //foo = 6; // Crash
        //bar = 6; // Allowed

    // (2*)
        //let x = 5;
        //let y = 5;

        //println!("x and y: {} and {}", x, y); //Similar to C String formatting

    // (3*)
        //read_line returns an io::Result which has the expect method
        //expect takes on thevalue it's called on, and if unsuccessful
        //panic! s (crashes, error handling)

    // (4*)
        //enum Foo {
          //Bar,
          //Baz,  
        //}
        // An enum in Rust, accessed by Foo::Bar or Foo:Baz
        // :: indicuates the namespace for the particular enum variant

    // (5*)
        // Rust was able to infer that guess should be a String

    // (6*)
        // Rust infered, by default, that secret_number is a
        // signed 32 bit integer (i32, unsigned is u32) 
    
    // (7*)
        // Rust allows 'shadowing' of the previous guess.
        // Never need to write guess_str AND guess for example.
        // In [guess.trim().parse()] guess refers to the old guess
        // which was a string.
        // trim() removes whitespace before and after the string.
        // The parse() method on strings parses a string into some
        // kind of number, which we need to tell Rust.
        // Notice the expect.
}