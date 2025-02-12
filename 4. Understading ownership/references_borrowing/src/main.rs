fn main() {
    let mut s1 = String::from("hello");

    {
        let r1 = &mut s1;
        println!("{r1}");
    }

    let r2 = &mut s1;
    println!("{r2}");

    change(&mut s1);
}

fn change(s: &mut String) {
    s.push_str(", world");
}
