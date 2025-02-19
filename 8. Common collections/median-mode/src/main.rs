use std::collections::HashMap;

fn main() {
    let list = vec![59, 1, 23, 81, 44, 29, 5, 76, 91, 99, 98, 73, 28, 49, 53, 17, 23, 45, 21];

    let median_value = median(&list);

    if let Some((key, value)) = mode(&list) {
        println!("Median value: {}", &list[median_value]);
        println!("The value that repeated the most was '{}', with {} repetitions!", key, value);
    } else {
        println!("There was an error :c");
    }
    
}

fn median(list: &Vec<i32>) -> usize {
    let mut new_list = list.clone();

    new_list.sort();

    let length = new_list.len() as f64;
    let half = (&length / 2.0).round();
    
    half as usize
}

fn mode(list: &Vec<i32>) -> Option<(&i32, i32)> {
    let mut map = HashMap::new();

    for n in list.iter() {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    let highest_map = map.iter().max_by_key(|&(_, v)| v).map(|(&k, &v)| (k, v));
    highest_map
}
