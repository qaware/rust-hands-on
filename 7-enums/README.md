# Enums

Kennt man, allerdings verhalten sie sich zwischen den Sprachen immer einwenig anders.

```
enum IpAddrKind { //Keyword Name-Of-Enum.
    V4, //Variants
    V6,
}
```

### Use enum:
```
let four = IpAddrKind::V4; //Enum-Name::Variant
```

### Enum + Values:

consider this case:

```
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

Ist doof wir könnten ein Ip-V6 deklarieren, die als String eine Ip-V4 beinhält... Wie können wir das Verbessern?

```
enum IpAddr {
    V4(u8, u8, u8, u8), //Variant(Datatypes)
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

Das ist schonmal besser aber immernicht optimal:

```
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Dem Enum isses erstmal egal was da reinkommt. Im Prinzip machen wir eine wieder eine Fachliche Klammer um die einzelnen Varianten, in dem Enum dingens, mit der wir wieder komfortabel arbeiter arbeiten könnten

### Enums + Methoden

But wait, there is more -> Methoden gibt es auch!

```
impl IpAddr {
    fn call(&self, message: String) {
        // method body would be defined here
    }
}

let address = ...;
address.write("Hello World");
```

## Wichtiges Std Enum -> `Option`

**In Rust there is no null** - the Billion Dollar Mistake. But sometime's it is useful to have null: a null is a value that is currently invalid or absent for some reason -> Option.

```
enum Option<T> {
    Some(T),
    None,
}
```

Type T macht es erstmal unabhängig für den Typen.

```
let some_number = Some(5); <-- type u32
let some_string = Some("a string"); <-- type string.

let absent_number: Option<i32> = None; <-- need funtion declartion for None.
```

Why is it better than null:

```
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

This will not compile, cause rust doesn't know how to add an i8 and Option<i8> with Null it would crash.

but do we access the Some, case? -> with match

https://doc.rust-lang.org/stable/book/ch06-02-match.html

## Using `match`

++ Strukturelles typematching.. 

`match` oder ein `switch` auf Koks.

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin { // keyword variable
        Coin::Penny => 1, //case-condition => {what to do},
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Hinweis, weder die einzelnen cases noch das match hat nen ; deswegen ist das ergebnis dieser Methode die einzelne u32-Zahl.

Der Compiler stellt sicher, dass alle möglichen Fälle beandelt werden! Der Code compiliert nicht, wenn das nicht der Fall ist!

Da der Code hier recht klein ist, können {} weggelassen werden, mehr code, würde die explizite Parenthesis benötigen.

## `match` + Values and numbers

Consider the following:
```
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

Jetzt müssen wir auf das zusätzliche Enum reagieren, wie machen wir das?

```
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

Wir deklarieren einfach einen Paramter in den `match`-Arm und arbeiten mit diesen weiter.

`match` funktioniert aber nicht nur mit enums sondern auch mit Zahlen!

```
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

Hier wird es schon schwieriger für alle Fälle Code zu finden. ein `default` kann über den Platzhalter `_` erreicht werden. Dieser kann allerdings auch in dem match-arm benutzt werden ->

```
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => { // we don't care.
            25
        },
    }
}
```
