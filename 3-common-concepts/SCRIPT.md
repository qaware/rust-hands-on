# 3 Common Programming Concepts
## 3.1 Variables and Mutability
Vorab: unused-Warnungen deaktivieren
```rust
#[allow(unused)]
```

Variablen deklarieren mit let:
```rust
let x = 5;
println!("The value of x is: {}", x);

x = 6;
println!("The value of x is: {}", x);
```
Immutable als Default -> keyword `mut`
```rust
let mut x = 5;
...
```
Konstanten ähnlich Java, müssen Typ-annotiert sein - bei variablen versucht der Compiler, den Typ zu infereieren
```rust
const MAX_POINTS: u32 = 100_000;
```
`u32` ist ein unsinged integer mit 32 bit, `_` für Lesbarkeit

Shadowing:
```rust
let x = x + 1;
let x = x * 2; 
```
Ist möglich, neues binding -> typ muss nicht mut sein.
```rust
let spaces = "    ";
let spaces = spaces.len();
```
Aber der Typ muss gleich sein!

## 3.2 Data Types

Wenn wir so was machen (unwrap ignorieren, kommt später):
```rust
let number = "42".parse().unwrap();
```
Müssen wir den Typ annotieren, weil das ergebnis von Parse unterschiedliche Typen haben kann!
```rust
let number: u32 = ...
```
Es gibt natürlich auch andere Integer:
Signed, Unsigned, 8,16,32,64 bits + isize/usize (von Architektur abhängig, für collections).
Zahlenwerte können auch Typ-annotiert werden und binär/hexadecimal etc. geschrieben werden.
```rust
let x: i8 = 42;
let y: u128 = 99999999999999999999999999999999999999999999999;

let z = 1usize;

let y = 0x42ff;
let x = 0b1111_0000;
let x = b'A';
```

Gleitkommazahlen, bools wie man es erwarten würde:
```rust
let flag = false;
let a = 2.0;
let b: f32 = 3.0;
```
`as` für Umwandlungen zwischen primitiven
```rust
let res = 1.0 / (2u8 as f32);
```

Chars sind unicode tokens, keine ascii Zeichen:
```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

### Zusammengesetzte Typen:
Tuples, lassen sich destrukturieren
```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
```
Oder per Index zugreifen:
```rust
let five_hundred = tup.0;
```

Arrays haben immer zur Compile-Zeit fixe Größe:
```rust
let arr = [1, 2, 3, 4, 5];
```
Typ ist `[typ; len]`, Zugriff wie Java:
```rust
let first = arr[0];
```
Was passier bei out-of-range? Einfache Fälle erkennt der Compiler:
```rust
let element = arr[10];
```
Aber komplizierte natürlich nicht -> Laufzeit-Fehler, und zwar ein expliziter panic! statt invalidem Zugriff.
```rust
let index = 10;
let element = arr[index];
```

## 3.3 Functions
Bisher:
```rust
fn main() { }
```
Funktionen in Rust haben snake_case namen, parameter werden mit `name: typ` angegeben:
```rust
fn another_function(x: i32, y: i32) {
    println!("The value of x and y is: {} and {}", x, y);
}
```
Funktionsaufruf wie in Java auch:
```rust
another_function(5, 6);
```
Wenn eine Funktion was zurückgibt, steht der Typ davon nach einem Pfeil:
```rust
fn five() -> i32 {
    5
}
```
Letzte Expression wird zurückgegeben, hier die 5.
Typischer Fehler: `;` am Ende -> `5;` ist ein Statement (wie Zuweisung), die Expression nach `;` wird zurückgegeben - hier ein leerer Typ/unit `()`.
Explizite Rückgabe auch möglich:
```rust
...
return 5;

```
Aber in Rust normalerweise nur um aus Branch zurückzuspringen.

## 3.4 Comments
Bis auf JavaDoc genau wie in java:
```rust
// Kommentar
/* Kommentar */

```

## 3.5 Control Flow
if/else: __Nur__ booleans als Bedingung (anders als in manchen Systemsprachen), keine Klammern notwendig:
```rust
let number = 3;

if number < 5 {
    println!("smaller 5");
} else {
    println!("greater 5");
}
```

Das Schöne: Alles Expressions, d.h. die letzte Expression aus jedem match-arm wird zurückgegeben:
```rust
let condition = true;
let number = (if condition {
    5
} else {
    6
});
```
Letztes Semikolon wegen Zuweisung!
Typ von den beiden Armen muss übereinstimmen, `else { "six" }` gibt einen Fehler.

### Loops
Drei Arten von loops:
While-Schleifen genau wie in Java:
```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);

    number = number - 1;
}
```

Besonders: Endlosschleife (können nur mit `break` beendet werden -> break kann Wert zurückgeben)
```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
```

Für collections (iterables)/ranges: for-in-loop
```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}
```

"Standard"-for loop durch ranges (`..`)
```rust
for i in 0..10 {
    println!("{}", i)
} 
```
Links inklusive, rechts exklusive.