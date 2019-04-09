# Common Collections:

## Vector

Creating:
```
let v: Vec<i32> = Vec::new();
// or:
let v = vec![1, 2, 3];
```
Add Values:
```
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

Read Values:

```
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

Unterschied ist, bei [] erhalte ich eine runtime-fehler beim Zugriff.
Bei get, erhalte ich NOne und kann weiterarbeiten.

Aufpassen mit Ownership:
```
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```
Results in
```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  --> src/main.rs:10:5
   |
8  |     let first = &v[0];
   |                  - immutable borrow occurs here
9  |
10 |     v.push(6);
   |     ^^^^^^^^^ mutable borrow occurs here
11 |
12 |     println!("The first element is: {}", first);
   |                                          ----- borrow later used here
```

Gefühlt sollte sowas funktionieren, aber warum tut es das nicht? Durch das push könnte ein neuer Speicher allokiert werden und das Element first, wird an eine andere Stelle abgelegt und könnte damit die in first gespeicherte Referenz kaputt machen!

Iterieren über die Werte:
```
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

// Kandidat zum löschen, wegen \*i
Iterieren und gleichzeitig die Werte verändern:
```
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```
// Kandidat zum löschen, wegen \*i

## Strings

The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

“strings” in Rust, they usually mean the `String` and the string slice `&str`

Unterschied &str und mutable.

### Create a new String

```
let mut s = String::new();

// or
let s = "initial contents".to_string();

//or
let s = String::from("initial contents");

//or
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

### Updating a String

With a method.
```
let mut s = String::from("foo");
s.push_str("bar"); // or with s.push('b');s.push('a');s.push('r');
// s is now "foobar"
```

With `+`
```
let s1: String = String::from("Hello, ");
let s2: String = String::from("world!");
let s3: String = s1 + &s2; // note s1 has been moved here and can no longer be used
```

okay what happens here? Method definition `fn add(self, s: &str) -> String `.
`add` consume self, and needs an reference to an string. which is what we do with `&s2` we transform `String to &String` the compiler transform's the `&String to &str`.

`add` doesn't consume s, which is why s2 is still usable but not s1.

With format macro
```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
// s is now tic-tac-toe
```

### Access Character.

```
let s1 = String::from("hello");
let h = s1[0];
```
In short this will fail, because Rust forbidds direct access. Because not every-character doesn't is only one u8-value.

To iterate the chars you can do:
```
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

And to iterate the bytes you can do:
```
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

### Summerize Strings
The Rust Book states that "strings are complicated".

## HashMaps!

### Creation and adding values

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Der Typ wird über die insert-Methode bestimmt, das erste ist String, das zweite i32.

### Accessing Values in HashMap

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```
note, Blue moved at first insertion.
note, get returns an Option<i32>.

### Iteration over key-value entries

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### Overwrite Values

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```
scores is 25

### Only insert if key has no values
```
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```
