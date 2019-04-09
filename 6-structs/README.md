# Structs

Structs bündeln Daten, die zusammen gehören.

## Code convention

`PascalCasing` für Name des Structs, `snake_casing` für die Felder.

## Definition:

```
struct User {         // keyword NameOfStruct
    username: String,	// feld_name: Typ
    email: String,
    sign_in_count: u64,
    active: bool, // Überflüssiges Komma ist erlaubt
}
```

### Instanz erstellen:

```
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

### Felder zugreifen:

```
user1.email = String::from("anotheremail@example.com");
```

Wenn user1 mut ist, dann ist das gesamte Struct mut. Rust hat kein Konzept, einzelne Felder als mut zu markieren.

### Factory-Methode, erster Wurf:

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

### Sugar:

Falls Parametername == Feldname, dann kann man das `field: ` weglassen.

```
fn build_user(email: String, username: String) -> User {
    User {
        email, // Feld-Name == Parameter-Name
        username, // Dito
        active: true,
        sign_in_count: 1,
    }
}
```

### Structs kopieren:

```
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1 // Kopiert übrige Felder aus user1
};
```

### Ausgeben des Structs:

```
// derive is an annotation
#[derive(Debug)]  //Here Voodoo happens, we will explain it later. For now we add additional functionality "Debug" to the Struct.
struct Rectangle {
  width: u32,
  height: u32
}

println!("rect1 is {:?}", rect1);
println!("rect1 is {:#?}", rect1); // Pretty print
```

### Methoden auf Structs

Bisher nur Daten, nun kommt Funktionalität: Methoden!

```
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle { // 0 - n impl Blöcke

  // Keyword Function-Name(optional self, optional Parameter-List)
  // Note: &self is the Type of impl, in this case &Rectangle.
  fn area(&self) -> u32 {
    self.width * self.height
  }

  // Mehrere Parameter
  fn can_hold(&self, other: &Rectangle) -> bool { 
    // ... 
  } 
}
```

### Methodenaufruf:

```
let rect1 = ...
let area = rect1.area();
```

**Wichtig ist:**
* Unterschied zwischen self, &self und &mut self (konsumieren, lesen, schreiben)
* Note für die C/C++ Programmierer -> Wir brauchen uns nicht um das Dereferenzieren kümmern, das macht Rust für uns.

**Vorteil gegenüber Functions:**
Wir sagen ganz klar, was man mit nem Rectangle machen kann. Die Methoden hängen nicht im luftleeren Raum.

### Associated Functions:

a.k.a. "Static Factories"

```
impl Foo {
    fn square(size:u32) -> Rectangle { // kein self, &self oder &mut self
		Rectangle {width: size, height: size}
	}
}

// ...
let square = Rectangle::square(3);
```