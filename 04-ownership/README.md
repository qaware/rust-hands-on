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

## 4.3 Slices
Damit können wir weiteren Datentyp verstehen: Slices. Sehr simpel, ähnlich Python:
```rust
let numbers = [1, 2, 3, 4, 5];
let one_to_three = &numbers[0..3];
let last = &numbers[3..];

println!("{:?} - {:?}", one_to_three, last);
```
Hier leihen wir uns 2x das Array, bzw. Sicht auf Teil davon (Überschneidung spielt keine Rolle - jeder borrow leiht das Array komplett, geht nur weil immutable!).
Bei Anfang/Ende könnnen wir uns Grenzen sparen. Funktioniert auch bei vielen anderen Typen (vecs, etc.).

### String slices
Besonders sind slices bei Strings - nicht verwunderlich, (UTF-8) Strings sind kompliziert!
```rust
let some_string = String::from("Hello world");

let H_char = some_string[0]; // compiler error

let world: &str = &some_string[6..];
``` 
Zunächst: Indexing geht nicht - denn es ist nicht klar, was zurückgegeben werden sollte (byte, codepoint, cluster).
Aber wir können String slicen, weil dann klar ist, wie viele Bytes.

Die Slice muss natürlich fixe Länge haben - deshalb ändert sich Type zu `&str` - String slice!
Das ist auch der Typ von String Literals, daher sind die auch nicht veränderlich.

Strings, die in der Größe unveränderlich sind, können wir also als `&str` speichern, veränderliche Strings als `String`.