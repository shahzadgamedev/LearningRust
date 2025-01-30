use rand::Rng;
use std::io;

pub fn guess_game_loop()
{
    println!("Hello, world!");

    println!("**** Guess Game **** ");

    let computer_guess: u32 = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut user_guess: String = String::new();
        println!("\nEnter Your Guess ..");
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to Read Line");
        println!("you Guessed {}", user_guess);

        let user_guess: u32 = match user_guess
            .trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please Enter a Valid Number.."); 
                    continue;
                }
            };

        match user_guess.cmp(&computer_guess) {
            std::cmp::Ordering::Less => println!("Too Small !!"),
            std::cmp::Ordering::Greater => println!("Too Big !!"),
            std::cmp::Ordering::Equal => 
            {
                println!("Dammmnn ! You Win");
                break;
            }
        }
    }

}