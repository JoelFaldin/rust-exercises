use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing values in a hashmap:
    {
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("{score}");
    }

    // Iterating over each key-value:
    {
        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }

    // Ownership on hash maps:
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);

        // here, field_name and field_value arent available!
    }

    // Updating a hash map:
    {
        // Overwritting a value:
        {
            let mut scores = HashMap::new();

            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Blue"), 25);

            println!("{scores:?}");
            // Original value overwritten!
        }

        //Adding key and value if key is not present:
        {
            let mut scores = HashMap::new();
            scores.insert(String::from("Blue"), 10);

            scores.entry(String::from("Yellow")).or_insert(50);
            scores.entry(String::from("Blue")).or_insert(50);

            println!("{scores:?}");
        }

        // Update value based on the old one:
        {
            let text = "hello world wonderful world";

            let mut map = HashMap::new();

            for word in text.split_whitespace() {
                let count = map.entry(word).or_insert(0);
                *count += 1;
            }

            println!("{map:?}");
        }
    }
}
