enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    }

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    }

    main3();
}

struct NewIppAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main2() {
    let home = NewIppAddrKind::V4(127, 0, 0, 1);
    let loopback = NewIppAddrKind::V6(String::from("::1"));
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMEssage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // custom method...
    }
}

fn main3() {
    let m = Message::Write(String::from("hello"));
    m.call();
}