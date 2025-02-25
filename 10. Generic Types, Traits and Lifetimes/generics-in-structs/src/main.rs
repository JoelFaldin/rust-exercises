struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x is {}", p.x());
}

struct Point_2<T, U> {
    x: T,
    y: U,
}

fn _main2() {
    let _both_integer = Point_2 { x: 5, y: 10 };
    let _both_float = Point_2 { x: 1.0, y: 4.0 };
    let _integer_and_float = Point_2 { x: 5, y: 4.0 };
}