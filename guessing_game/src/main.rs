use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number dude!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Input your guess here: ");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line :C");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Too small!"),
            Ordering::Greater => print!("Too big!"),
            Ordering::Equal => {
                print!("You won bro :D");
                break;
            },
        }
    }
}