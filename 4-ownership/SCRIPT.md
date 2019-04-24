# 4 Ownership
Bis jetzt die meisten Konzepte auch aus anderen Sprachen bekannt.
Fundamental anders und für Speicherverwaltung grundlegend: Ownership. Bedeutet, dass jedes Objekt irgendjemandem "gehört".
An Beispiel, veränderlicher String:
```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1); // compiler error
```
Das geht nicht! String gehört s1.
Zuweisung verändert Owner ('move'), string gehört s2.
Über s1 nicht mehr zugreifbar, weil s1 nicht owner ist - einzig und allein s2 ist owner!

Möglichkeit das zu beheben: Kopie:
```rust
...
let s2 = s1.clone();
```

Twist: Ganze nur halb wahr. Funktioniert einwandfrei:
```rust
let i1 = 5;
let i2 = i1;

println!("{}", i1);
```

Grund: Integer-Typ so einfach, dass er bei Zuweisung kopiert wird (4 bytes, fix, liegt auf stack).
Man sagt, diese Typen sind 'copy'.

Komplexere Typen (veränderlicher String) nicht so einfach zu kopieren - Größe unbekannt -> muss auf Heap liegen.
Kopieren wäre Mehraufwand -> werden "bewegt"!

### Functions
Was bei Funktionsaufrufen? Selbes Problem:
```rust
fn calc_len(s: String) -> usize {
    s.len()
}
```
String danach tot! Möglichkeit:
```rust
fn calc_len_and_ret(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
...
let (s3, len) = calc_len_and_ret(s2);

```
Aber viel Aufwand! Bequemere Möglichkeit: String nur ausleihen (borrowing)!
## 4.2 References Borrowing
Dazu Typ mit `&` annotieren:
```rust
fn calc_len_borrow(s: &String) -> usize {
    s.len()
}
...
let len = calc_len_borrow(&s1);
println!("Length of {}: {}", s1, len);
```
Referenz wird Methode übergeben, und sobald die Reference out of scope geht (hier: Methode beendet), ist Leihgabe beendet.

References sind genau wie Variablen per Default immutable und werden per `mut` mutable:
```rust
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
...
let mut s = String::from("hello");

change(&mut s);
```
Achtung: Es kann beliebig immutable borrows geben ODER ein einziges mutable borrow.
```rust
let r1 = &mut s;
let r2 = &s; // compiler error

println!("{}, {}", r1, r2);
```

## 4.4 Slices
Damit können wir weiteren Datentyp verstehen: Slices. Sehr simpel, ähnlich Python:
```rust
let my_string = String::from("Hello world");
let hello = &my_string[0..5];
let world = &my_string[6..];

println!("{}, {}!", hello, world);
```
Hier leihen wir uns 2x den String, bzw. Sicht auf Teil des Strings (Überschneidung spielt keine Rolle - jeder borrow leiht String komplett, geht nur weil immutable!).
Bei Anfang/Ende könnnen wir uns Grenzen sparen. Funktioniert auch bei vielen anderen Typen (arrays, vecs, etc.).