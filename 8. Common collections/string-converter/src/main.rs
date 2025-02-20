fn main() {
    let string = String::from("this is a test broski");
    let separated_string: Vec<&str> = string.split(" ").collect();

    let result: Vec<String> = separated_string
        .iter()
        .map(|word| word_to_pig_latin(word))
        .collect();

    println!("{}", result.join(" "));
}

fn word_to_pig_latin(word: &str) -> String {
    let vowels = vec!["a", "e", "i", "o", "u"];
    
    let first_letter: &str = &word[0..1];
    
    if vowels.contains(&first_letter) {
        let s = format!("{word}-hay");
        s
    } else {
        let temp_s = &word[1..];

        let s = format!("{temp_s}-{first_letter}ay");
        s
    }
}