fn main() {
    // Loop:
    loop {
        println!("Again!");
        break;
    }

    // Return values from loop:
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is: {result}");

    // Multiple loops:
    let mut count = 0;
    'countin_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'countin_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // Conditionals with while:
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!");
    
    // Looping through a collection:
    let b = [10, 20, 30, 40, 50];

    for element in b {
        println!("Element is: {element}");
    }

    // Liftoff v2:
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}
