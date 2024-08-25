use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game 2024 by Hiro");

    let secret_number = rand::thread_rng().gen_range(0..100);

    // println!("Secret No: {}", secret_number);

    loop {
        let mut guess = String::new(); //guess is appended not replaced by  stdin();  do guess.clear () other way around

        println!("{}", "Input ur guess".blue());

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = guess.trim().to_string();

        if guess == "q" || guess == "quit" {
            println!("quitting");
            break;
        }

        let guess: u32 = guess.parse().expect("Please type a number");

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".yellow()),
            Ordering::Equal => {
                println!("{}", "Correct value".green());
                break;
            }
            Ordering::Greater => {
                println!("{}", "Value is big".red());
                println!("{}", "Enter a smaller value".on_bright_red());
            }
        }
    }

    println!("end");
}
