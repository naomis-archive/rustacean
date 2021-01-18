use rand::Rng; // Hurray we installed a package!
use std::cmp::Ordering;
use std::io; // input/output stuff.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        /*
         *Okay lotsa stuff in this line...
         * So the "mut" makes this a mutable variable
         * The :: is used to access the `new` function of the String type
         */
        let mut guess = String::new();

        /*
         * ::stdin() is grabbing the input function from io
         * &mut guess is creating a reference
         * .expect is handling the error.
         */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        /*
         * Here we convert the guess into an integer
         * We can reassign it - it's called shadowing
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        /*
         * Here the {} serves as a placeholder for guess
         * Can use multiple {} and multiple variables
         */
        println!("You guessed: {}", guess);

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
