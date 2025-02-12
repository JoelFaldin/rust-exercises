use std::io;

fn main() {
    println!("Welcome to the nth-fibonacci-number!");
    println!("Enter the n-number you want to get: ");
    println!("------------------------------------");

    let mut true_prev = 0;

    let mut previous = 0;
    let mut current = 1;

    for n in 0..11 {
        print!("{previous} ");

        if n != 1 {
            true_prev = current;
        }

        current = previous + current;
        previous = true_prev;
    }

    println!("");
    println!("Enter the number you want: ");
    
    let mut numb = String::new();

    io::stdin()
        .read_line(&mut numb)
        .expect("Enter a valid value!");

    let mut num: u32 = match numb.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut prev = 0;

    let mut prev1 = 0;
    let mut current1 = 1;

    num -= 1;

    for n in 0..num {
        if n != 1 {
            prev = current1;
        }

        println!("{current1}");
        current1 = prev1 + current1;
        prev1 = prev;
    }

    println!("The {num}nth number in the fibonacci series is: {current1}");
}
