use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // this will create an infinite loop
    loop {
        println!("Please input your guess.");

        //variables are immutables we use mut to make them mutables.
        //:: means that new is a function associate with String.
        // we also need to parse this value to number, we can:
        // i32 32-bit number
        // u32 unsigned 32-bit number
        // i64 64-bit number, etc.
        let mut guess = String::new();

        // we can use io:: because we imported it by using `use std::io`
        io::stdin()
            // we are referencing the variable guess
            // references are immutables so we need to use &mut instead of &guess
            // so we can mutate thet value.
            // this return a `Result` that it's an `enum` that can have multiple possible states or variant
            // Result are `Ok` and `Err`
            .read_line(&mut guess)
            .expect("Failed to read line");

        // we are shadowing the guess variable so we can reuse.
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // after the assertion we can, stop the loop
                break;
            },
        }
    }
}
