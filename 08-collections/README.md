# Common Collections:

Arrays und Tuples speichern die Daten auf dem Stack und haben eine feste Größe. Die nachfolgenden Collections
speichern die Daten im Heap und können wachsen und schrumpfen.

## Vector

### Erstellen
```
let v: Vec<i32> = Vec::new(); // Leerer Vector
// Oder...
let v = vec![1, 2, 3]; // Vector, der 1 2 3 enthält
```

### Werte hinzufügen
```
let mut v = Vec::new(); // Muss mut sein, sonst funktioniert .push nicht

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### Werte lesen

```
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

Unterschied: 
- bei `[]` kommt ne panic bei index out of bounds.
- `.get()` gibt ein `Option` zurück.

Aufpassen mit Ownership, auch bei Lesen:
```
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

Funktioniert nicht:

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

Aber warum geht das nicht? -> Durch `.push` könnte ein neuer Speicher allokiert werden, in dem die bisherigen Elemente kopiert werden. Dann ist das erste Element an einer anderen Stelle im Speicher und die Referenz in `first` ungültig.

### Iterieren über die Werte
```
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

## Strings

Strings sind mutable und UTF-8 kodiert. Das macht das Arbeiten mit ihnen etwas mühsam.

Strings: `String` und `&str`.

### Erstellen

```
let mut s = String::new();

// oder
let s = "initial contents".to_string();

// oder
let s = String::from("initial contents");

// UTF-8 erlaubt
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

### Anhängen

```
let mut s = String::from("foo");
s.push_str("bar"); // oder s.push('b'); s.push('a'); s.push('r');
// s = "foobar"
```

Mit `+`
```
let s1: String = String::from("Hello, ");
let s2: String = String::from("world!");
let s3: String = s1 + &s2; // s1 kann nun nicht mehr verwendet werden
```

Wieso?!

`+` ruft `fn add(self, s: &str) -> String` auf. `self` (s1) wird konsumiert, der zweite String (s2) wird nur als Referenz gelesen. Deswegen ist `s1` nun nicht mehr zugreifbar, `s2` hingegen schon.

Alternative:
`format!`:

```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
// s = tic-tac-toe
// s1 s2 und s3 sind noch nutzbar
```

### Einzelne Zeichen (characters) lesen.

```
let s1 = String::from("hello");
let h = s1[0];
```
Geht nicht. Nicht jeder Character ist nur ein u8 lang.

Chars:
```
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

Bytes:
```
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

Das macht vermutlich immer noch nicht das, was ihr wolltet. Um wirklich jedes sichtbare Zeichen auszugeben, müsst ihr die Grapheme iterieren. Dafür gibt's Crates, die sowas können.

### Konvertieren

`&str` zu `String`: `s.to_string()`

`String` zu `&str`: `s.as_str()`

### TL;DR
"Strings are complicated".

Rust hat einen anderen Trade-Off als Java gewählt: Java versucht, Encoding vom Benutzer zu verbergen, und scheitert daran ab und zu, gerade wenn man mit Zeichen aus dem fernöstlichem Raum arbeitet.
Rust versucht nicht, die Komplexität zu verstecken.

Ein Beispiel:

"नमस्ते" sind 4 sichtbare "Zeichen". In UTF-8 sind das die Bytes `[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]`. Wenn man das in `char` parst, kommen immer noch 6 Zeichen
`['न', 'म', 'स', '्', 'त', 'े']` raus. Das 4. und das 6. sind keine "Zeichen", die alleine stehen können. Die Grapeheme sind das, was am intuitivsten ist: `["न", "म", "स्", "ते"]`.
Man hätte nun auch die Strings in UTF-32 speichern können, aber das verbraucht für viele Strings in unserem Raum 4x so viel Speicher wie nötig. Aber selbst UTF-32 würde nicht das Problem mit den Graphemen lösen.
Strings sind einfach kompliziert.

## HashMaps

### Erstellen

```
use std::collections::HashMap;

let mut scores = HashMap::new(); // Leere Hashmap

scores.insert(String::from("Blue"), 10); // "Blue" -> 10
scores.insert(String::from("Yellow"), 50); // Yellow -> 50
```

Der Typ wird über die insert-Methode bestimmt, das erste ist `String`, das zweite `i32`.

### Lesen

```
let team_name = String::from("Blue");
let score = scores.get(&team_name); // score: Option<i32>
```

### Einträge iterieren

```
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### Werte überschreiben

```
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores); // Blue -> 25
```

### Wert nur überschreiben, falls noch nicht vorhanden
```
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores); // Blue -> 10, Yellow -> 50
```
