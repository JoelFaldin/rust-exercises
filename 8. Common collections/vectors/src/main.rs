fn main() {
    // Creating a new vector:
    {
        let _v: Vec<i32> =  Vec::new();

        let _v2  = vec![1, 2, 3];
    }

    // Updating a vector:
    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    // Reading elements of a vector:
    {
        let v = vec![1, 2, 3, 4, 5];

        // Indexing syntax:
        {
            let third = &v[2];
            println!("Third element: {}", third);
        }

        // Get method:
        {
            let third = v.get(2);
            match third {
                Some(third) => println!("Third element is {third}"),
                None => println!("No third element :c"),
            }
        }
    }

    // Reading nonexistant elements in a vector:
    {
        let v = vec![1, 2, 3, 4, 5];

        let does_not_exists = &v[100];
        let does_not_exists = v.get(100);
    }
}
