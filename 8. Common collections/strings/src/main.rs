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
    }
}
