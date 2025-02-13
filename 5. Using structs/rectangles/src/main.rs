fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );


    let rec1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rec1)
    );

    main1();
    main2();
    main3();
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}


fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main1() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1); // Without the :?, it wont load
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}

fn main3() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    println!("rect1 is {rect1:?}");
}