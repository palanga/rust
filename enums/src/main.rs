fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}", four);
    println!("{:?}", six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };


    println!("{:?}", home);

    let home2 = IpAddress::V4(String::from("127.0.0.1"));

    println!("{:?}", home2);

    let home3 = IpAddress2::V4(127, 0, 0, 1);

    let some = Option::Some(1);

    let some_plus_one = some.map(|n| n + 1);

    println!("{:?}", some);
    println!("{:?}", some_plus_one);

    if let Some(number) = some {
        println!("{}", number)
    };

    let message: Message = unimplemented!();

    match message {
        Message::Quit => {}
        Message::Move { .. } => {}
        Message::Write(_) => {}
        // Message::ChangeColor(_, _, _) => {}
    }

}

// #[derive(Debug)]
// enum Option<T> {
//     None,
//     Some(T),
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum IpAddress2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
