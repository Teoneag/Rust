use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to THE guessing game!");
    println!("In order to win, you need to guess the number I'm thinking about");
    println!("For now, you have unlimited tries. Good luck!");
    
    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Please enter a valid number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}