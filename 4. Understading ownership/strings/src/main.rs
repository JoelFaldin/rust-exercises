fn main() {
    let mut s = String::from("Hello");
    
    s.push_str(", world!");

    println!("{}", s);

    println!("------------------");
    
    let x = 5;
    let y = x;

    println!("{x}");
    println!("{y}");

    println!("------------------");
    
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}");
    println!("{s2}");

    println!("------------------");

    let s3 = String::from("Hello");
    let s4 = s3.clone();

    println!("{s3}");
    println!("{s4}");
}
