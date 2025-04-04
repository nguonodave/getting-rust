use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is a guessing game!");
    println!("Guess a number between 1-100");

    let secret = rand::rng().random_range(1..=101);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("msg");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        }
    }
}
