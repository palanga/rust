fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("some@email.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("someother@email.com");

    let user2 = build_user(String::from("lala"), String::from("lalala"));


    println!("{:#?}", user1);
    println!("{:?}", user2);

    let user3 = User {
        email: String::from("lalala@lala.com"),
        ..user2
    };

    println!("{:?}", user3);
    dbg!(user3);
    // println!("{:?}", user2); // no compila


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rectangle = Rectangle { width: 10, height: 5 };
    let area = rectangle.area();
    println!("{}", area);

    let square = Rectangle::square(4);
    println!("{:?}", square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct AlwaysEqual;

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
