use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secrete number is: {}", secret_number);
    loop {
        println!("Please input you guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("filed to read line");

        let guess: u32 = guess.trim().parse().expect("please type a  valid number");
        println!("your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too large!"),
        }
    }
}
