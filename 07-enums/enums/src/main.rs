enum IpAddr {
    V4(u8, u8, u8, u8), // Hat 4x u8
    V6(String), // Hat 1x String
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    home.call("Hello World");

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // Doesn't compile
    // let sum = x + y;

    match some_number {
        None => println!("Leer"),
        Some(s2) => println!("Was drin: {}", s2)
    }

    let value = value_in_cents(Coin::Quarter);
    println!("value: {}", value);
}

fn value_in_cents(coin: Coin) -> u32 {
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
}

impl IpAddr {
    fn call(&self, message: &str) {
        // ...
    }
}