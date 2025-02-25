fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("Largest number is: {result}");

    let char_list = vec!['y', 'm', 'p', 'a'];
    
    let result = largest(&char_list);
    println!("The largest char is {result}");
}
