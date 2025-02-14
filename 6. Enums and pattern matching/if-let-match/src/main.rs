fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("Max configured to be {max}"),
        _ => (),
    }

    main2();
    main3();
}

fn main2() {
    let config_max = Some(3u8);
    
    if let Some(max) = config_max {
        println!("The max is configured to be {max}");
    }
    // Code in if let isnt run if value doesnt match the pattern!
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main3() {
    let mut count = 0;
    
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
}

fn main4() {
    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}