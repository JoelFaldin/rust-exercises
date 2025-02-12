fn main() {
    let number = 7;

    if number < 5 {
        println!("condition is true!");
    } else {
        println!("condition is false!");
    }

    let number2 = 6;

    if number2 % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number2 % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number2 % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }

    let condition = true;
    let number3 = if condition { 5 } else { 6 };

    println!("Value of number3 is: {number3}");
}
