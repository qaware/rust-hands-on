
# Structs

Nur mit Primitiven zu arbeiten wäre ziemlich mühseelig, deswegen gibts die guten alten Structs um Daten, die zusammen gehören auch im Code zu bündeln.

## Defining a struct:

```
struct User {         //keyword Name-Of-Struct
    username: String,	//Field-Name: Type-Of-Field
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

### Create a struct:

```
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

### Access fields:

```
user1.email = String::from("anotheremail@example.com");

```

### Example of Factory-Method:

```
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

### Suggar:

Convient Shorthand: If parameter name + struct field the same, so it can be skipped.

```
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```
Create Instance from other instances
```
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

Einfaches Ausgeben des Structs:

```
((derive is an annotation))
#[derive(Debug)]  //Here Voodoo happens, we will explain it later. For now we add additional functionality "Debug" to the Struct.
struct Rectangle {
}

println!("rect1 is {:?}", rect1);
```

## Defining Methods for a Structs

All Fun and Games, but aren't we missing something? Yes -> Methods, are functions decladed within the context of a struct.

```
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle { //Keyword Struc-Name.

  // Keyword Function-Name(optional self, optional Parameter-List)
  // Note: self is the Type of impl, in this case &Rectangle.
  fn area(&self) -> u32 {
    self.width * self.height
  }
}
```

### Method-Call:

```
let rect1 = ...
rect1.area();
```

**Wichtig ist:**
* Unterschied zwischen self, &self und &mut self (das erste nimmt die Ownership, immutable referenz und mutable ref.)
* Note für die C/C++ Programmierer -> Wir brauchen uns nicht um das deferenzieren kümmern, das macht Rust für uns.

**Vorteil Method's gegenüber Functions:**
Wir sagen ganz klar, was man mit nem Rectangle machen kann.

### Associated Functions:

E.g. Static Factories.

```
impl Foo {
    fn square(size:u32) -> Rectangle {
		Rectangle {width: size, height: size}
	}
}

...
let square = Rectangle::square(3);
...

```
