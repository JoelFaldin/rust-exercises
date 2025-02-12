fn main() {
    let my_string = String::from("hello world");

    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);
    let _word = first_word(&my_string_literal);

    other_slices();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3])
}