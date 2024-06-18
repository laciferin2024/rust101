use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guessing game 2024 by Hiro");

    println!("Input ur guess");

    let secret_number = rand::thread_rng().gen_range(0..100);


    // println!("Secret No: {}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Correct value");
                break;
            },
            Ordering::Greater => {
                println!("Value is big");
                println!("Enter a smaller value");
            }
        }
    }


    println!("end");
}