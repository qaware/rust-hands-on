Cargo
=====
 ```jshelllanguage
cargo new rust-intro
```
`Cargo.toml` zeigen
```toml
[package]
name = "rust-intro"
version = "0.1.0"
authors = ["f.huch"]
edition = "2018"

[dependencies]
```
`main.rs` zeigen
```rust
fn main() {
    println!("Hello, world!");
}
```
`fn` deklarierte Funktion, kein return type (`void` in java).

`println!` ist ein Makro zum schreiben auf stdout

Ausführen: `cargo check`, `cargo build`, `cargo run`