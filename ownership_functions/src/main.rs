fn main() {
    let s = String::from("Hello");

    takes_ownership(s);
    // Here, s is moved to the function, no longer invalid here

    let x = 5;
    // Here, it should be moved, but i32 is copy so its fine :D
    
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
