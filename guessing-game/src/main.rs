use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // gen_range() is inclusive on the lower bound and exclusive on the upper bound.
    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess a number:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading user input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("The input must be a number.");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("{} is to small.", guess),
            Ordering::Greater => println!("{} is to high.", guess),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
