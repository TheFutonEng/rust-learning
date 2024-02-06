use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input our guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number: ");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")

    }
}


// The standard Rust library is brought into this program (std) because the IO fuction is needed. The standard library documentaiton is called the prelude and is located at the link below:

// https://doc.rust-lang.org/std/prelude/index.html

// Again, the 'main' function is the entry point to any program.  The 'fn' declares the start of a function, the blank parenthesis indicate that there are no arguments and the brackets contain the function body.

// AS we learned previously, 'println!' represents a macros (still not clear on what that is)

// Line 8 creates and stores a value as a variable.  In Rust, variables are immutable by default.  The presense of the keyword 'mut' on line 8 is what makes the variable 'guess' mutable.

// The '=' sign in line eight binds the right side to the left side variable label.  'String::new()' is a function that returns a new instance of a String which is a type provided by the standard library.

// The '::' syntax in this line means that 'new()' is an assiciated function of the String type.  An associated function is a function that's implemented on a type, in this case a String.

// Lines 10-12 are somewhat complicated.  First off, line 1 imports the io library which is why line 10 can start with 'io::stdin()'.  If it hadn't been imported, this call could still be made by instead calling 'std::io::stdin()'.  Function docs can be found here - https://doc.rust-lang.org/std/io/fn.stdin.html

// Line 11 calls the 'read_line' method of a standard input handle (io::stdin) to get input from the user.  This method is also being passed an argument of '&mut guess' to tell it what string to store the input from the user.

// The '&' in line 11 means that a reference to 'guess' is what is being passed.  Rust makes it very safe to pass references around which can be tricky in other languages.  References in rust are immutable by default as are variables.

// Line 12 finally finishes the expression that began on line 10 (these lines could have been written out on one long line).

// Rust Arms are kind of like switch statements in other languages.