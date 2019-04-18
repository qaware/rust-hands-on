# Enums

Aufzählungstypen. Sind in allen Sprachen etwas anders, natürlich auch in Rust.

## Definition
```
enum IpAddrKind {
    V4, // Werte des Enums
    V6, // Trailing comma erlaubt
}
```

## Verwendung
```
let four = IpAddrKind::V4;
```

## Werte

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

Enum + Struct. Wäre besser, wenn das alles in einem wäre.

## Nächster Versuch mit den Werten

```
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4("127.0.0.1");
let loopback = IpAddr::V6("::1");
```

Schon besser. Leider geht nun sowas:

```
let home = IpAddr::V4("garantiert keine IPv4 adresse");
```

## Und noch ein Versuch

```
enum IpAddr {
    V4(u8, u8, u8, u8), // Hat 4x u8
    V6(String), // Hat 1x String
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

Schon besser. Enums können also Daten haben, und sogar ihr Aussehen je nach Enumeration ändern. Das geht z.B. in Java nicht.

Das geht mit allen Wertetypen, auch mit Structs:

```
struct Ipv4Addr {
    // ...
}

struct Ipv6Addr {
    // ...
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
## Enums + Methoden

Methoden gibt's (natürlich) auch.

```
impl IpAddr {
    fn call(&self, message: String) {
        // ...
    }
}

let address = IpAddr::V4(127, 0, 0, 1);
address.call("Hello World");
```

## Wichtiges Enum -> `Option`

Option als Ersatz für null. **ES GIBT KEIN NULL IN RUST**.

So ist `Option` in der Standardlibrary definiert:

```
enum Option<T> {
    Some(T),
    None,
}
```

Type `T` macht es erstmal unabhängig für den Inhalt. Kann man sich wie eine Box vorstellen, die entweder `T` enthält oder leer ist. 

```
let some_number = Some(5); // some_number ist Option<i32>
let some_string = Some("a string"); // some_string ist Option<&str>

let absent_number: Option<i32> = None; // Nun braucht man einen expliziten Typen
```

Wieso ist das nun besser als null?

```
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

Das kompiliert nicht, weil `x` ein `i8` und `y` ein `Option<i8>` ist. Das Typsystem fängt den Fehler beim Kompilieren anstatt dich nachts anzurufen.

## Match

Wie arbeiten wir nun mit so einem `Option`?

```
let s = Some(12); // Option<i32>

match s {
    None => println!("Leer"),
    Some(s2) => println!("Was drin: {}", s2) // s2 ist nun ein i32
}
```

`match` ist ein `switch` auf Koks.

Weiteres Beispiel:

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
}
```

Match kann einen Wert zurückgeben. Der Compiler stellt sicher, dass alle möglichen Fälle beandelt werden! Der Code compiliert nicht, wenn das nicht der Fall ist.

`match` funktioniert aber nicht nur mit enums sondern auch mit Zahlen!

```
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (), // Default case
}
```

Dieses `_` kann man immer dann verwenden, wenn einem der Wert einer Variable egal ist:

```
let s = Some(12); // Option<i32>

match s {
    None => println!("Leer"),
    Some(_) => println!("Was drin")
}
```
