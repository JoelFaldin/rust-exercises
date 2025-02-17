fn main() {
    let data = "initial content bro";

    let _s = data.to_string();
    // This also works -> "initial content".to_string()

    // Equivalent:
    {
        let _s = String::from("initial content bro");
    }

    // Updating a string:
    {
        // Appending to a string with push_str and push:
        {
            let mut s = String::from("foo");
            let s2 = "bar";
            s.push_str(s2);

            println!("{s}");
            println!("{s2}");

            let mut s3 = String::from("wha");
            s3.push('t');

            println!("{s3}");
        }

        // Concatenating strings:
        {
            let s1 = String::from("hello");
            let s2 = String::from(" world!");
            let s3 = s1 + &s2;

            println!("{s3}");
        }

        {
            // Alternative (using format!):
            {
                let s1 = String::from("hello");
                let s2 = String::from("rust");
                let s3 = String::from("world!");

                let s4 = format!("{s1} {s2} {s3}");
                println!("{s4}");
            }
        }
    }

    // Iterating over strings:
    {
        for c in "test".chars() {
            println!("{c}");
        }

        for c in "Зд".chars() {
            println!("{c}");
        }

        for c in "はじめまして".chars() {
            println!("{c}");
        }
    }
}
