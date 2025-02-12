fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // println!("{hello}");
    // println!("{world}");

    // Equals:
    let s1 = String::from("hello");

    let len = s1.len();

    let _slice1 = &s[3..len];
    let _slice2 = &s[3..];

    // Also equals:
    let _slice3 = &s[0..len];
    let _slice4 = &s[..];

    prev_main();
}

fn prev_main() {
    let mut s = String::from("Hello world");
    let word = new_first_word(&s);

    println!("{word}");

    s.clear();

    // println!("{s}");
}

fn new_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn old_first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }