use std::io; 
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number from 1-100!");
    println!("But remember you have only 5 tries!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut counter = 0;
    
    while  counter < 5{
        println!("Please enter your guess:");
        let mut guess = String::new();

        counter += 1;

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // converts the string to 32-bit unsigned number
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },       
        }
    };

    if counter == 5 {
        println!("You lost... Try again!")
    };
    
}
