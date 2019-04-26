struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let rect1 = Rectangle { width: 2, height: 3 };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    let area = rect1.area();

    let square = Rectangle::square(1);

    let rect2 = Rectangle::new(1, 2);
    println!("rect2 is {:?}", rect2);

    let can_hold = rect1.can_hold(&rect2);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}
