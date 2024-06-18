use std::cmp::Ordering;
use std::io;
use std::io::Read;
use rand::Rng;

fn main(){
    println!("Guessing game 2024 by Hiro");

    println!("Input ur guess");

    let secret_number = rand::thread_rng().gen_range(0..100);

    let mut guess = String:: new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("Your guess: {}",guess);

    println!("Secret No: {}", secret_number);

    match guess.cmp(&secret_number){
        Ordering::Less=> println!("Too small"),
        Ordering::Equal=> println!("Correct value"),
        Ordering::Greater=>{
            println!("Value is big");
            eprintln!("Enter a smaller value");
        }

        _ => {
            eprintln!("invalid cmp")
        }
    }

    println!("end");

}