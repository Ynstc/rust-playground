use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is {}", secret_number);

    loop {
        let mut guess =  String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            let guess = match guess.trim().parse::<u32>() {
                Ok(num) => num,
                Err(_) => continue
            };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".italic().red()),
            Ordering::Greater => println!("{}", "Too big!".bold().red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
        }

    }
}
