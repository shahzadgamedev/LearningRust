use std::io;
use rand::Rng;

mod qunatum_comp;

fn main() {
    println!("Hello, world!");

    println!("**** Guess Game **** ");
    

    let mut user_guess: String = String::new();
    let computer_guess = rand::thread_rng().gen_range(1..=100);


    io::stdin().read_line(&mut user_guess).expect("Failed to Read Line");
    println!("you Guessed {}", user_guess);
 


}
