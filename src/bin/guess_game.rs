use std::io;
use std::io::Read;
use rand::Rng;

fn main(){
    println!("Guessing game 2024 by Hiro");

    println!("Input ur guess");

    let secret_number = rand::thread_rng().gen_range(0..100);

    let mut guess = String:: new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("Your guess: {}",guess);

    println!("Secret No: {}", secret_number);

    println!("end");

}