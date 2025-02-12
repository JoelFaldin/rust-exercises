fn main() {
    let x = 5;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Tuple: (values can have different types)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("{x}");
    println!("{y}");
    println!("{z}");

    let five_hundred =  tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{five_hundred}");
    println!("{six_point_four}");
    println!("{one}");

    // Arrays: (every element must have the same type)
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    println!("{first}");
    println!("{second}");
}
