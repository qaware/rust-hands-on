# 11 Testing
## 11.1 Unit tests
Testing in rust/cargo integriert!
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_stuff() {
        assert_eq!(2 + 2, 5); // Failing test
    }
}
```
In rust liegen tests im selben File iwe das Modul - mit cfg test, um vom Code zu unterscheiden.
Ausführen mit `cargo test`. 
Abseits von `assert_eq!` gibt es noch `assert_ne!` und `assert!`, sonst externe crates.

Um zu testen ob der code auch richtig `panic`ed:
```rust
#[test]
#[should_panic]
fn test_arithmetic() {
    let zero = 1 - 1;
    let singularity = 1 / zero;
}

```
Dabei festlegen, warum `panic`:
```rust
#[shoud_panic(expected="divide by zero")]
```
Der `panic`-Grund muss "divide by zero" enthalten.

Meistens aber eher results, deshalb direkt damit testen:
```rust
fn get_result() -> Result<(), String> {
    Err(String::from("something happened"))
}

...

use crate::get_result;

#[test]
fn test_result_ok() -> Result<(), String> {
    get_result() // Test fails
}
```

## 11.2 Integration tests
Integration tests kommen in den test/ ordner.
Sind eigene module, die library normal benutzen:

`src/lib.rs`:
```rust
pub fn some_func(x: u32) -> u32 {
    x * (x - 1) * (x - 2)
}
```
`tests/integration_test.rs`:
```rust
use rust_intro::some_func;

#[test]
fn test_stuff() {
    assert_eq!(some_func(3), 6)
}
```

Per Konvention liegen integration tests unter `tests/`.
`#[cfg(test)]` nicht nötig, da hier nur tests liegen.
Integration tests können keine privaten Methoden aufrufen, deshalb `pub`.
