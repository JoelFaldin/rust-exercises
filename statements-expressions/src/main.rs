fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("Value of x is: {x}");
}

fn five() -> i32 {
    // Returning the value "5":
    5
}