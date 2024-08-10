use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); // Inclusive on both ends

    loop {
        println!("Please input your guess.");

        // Variables are immutable by default. Use `let` to create a variable.
        // `String::new` is a function, returns a new instance of `String`.
        // The `::` indicates that it is an associated function of the `Stringh` type.
        let mut guess: String = String::new();

        io::stdin() // If std::io is not imported, we could still use the function with std::io::stdin()
            .read_line(&mut guess) // read_line is a method of standard input handle. And store the read line in guess.
            // read_line() take whatever the user types into the standard input and append into a string
            // & indicates that this argument is a reference so not the entire data is being copied into memory
            // references are immutable by default
            // &mut guess rather than %guess
            .expect("Failed to read line"); // Handling Potential Failure
                                            // read_line returns a Result type of Ok or Err
                                            // if expect is not called, and program will still compile and there will be a warning
                                            // suggesting Result may be an Err variant, which should be handled

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // Err(_) is a catch all pattern.
                println!("Please type a number!");
                continue;
            }
        };
        // although guess is already declared. Rust allows us to shadow the previous value of guess witrh a new one
        // parser() will use the : u32 to parse the string into an integer
        // u32 is a 32-bit unsigned integer

        println!("You guessed: {}", guess);
        // {} is a placeholder for the value of guess

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // Ordering is an enum, having three variants: Less, Equal, Greater
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
