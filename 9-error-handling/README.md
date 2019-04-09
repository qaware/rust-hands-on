# Error Handling

Natürlich funktioniert das auch wieder anders, als wir kennen.

In Rust werden erstmal zwei Arten von Fehlern unterschieden. Recoverable Fehler, wie ich habe eine Datei nicht gefunden. Oder Unrecoverable Fehler wie Bugs, OutOfBoundsException.

Es gibt KEINE Exception, es gibt max. einen Return-Type Result<T,E> für die recoverable Fehler und [panic!] für unrecoverable Fehler.

https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html


## PANIC!! on the titanic!

```
fn main() {
  panic!("Frauen und Kinder zu erst!");
}
```

Results in
```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
Now we have the thread, the panic message and the responsible code-line.


A real life example:
```
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

In C this would compile and run.. In an real program this would corrupt our memory. yet in Rust you fail immediately.

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
99', /checkout/src/liballoc/vec.rs:1555:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

But now the panic message doesn't help so we add RUST_BACKTRACE=1 to our environment variable and we get the following output:

```
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /checkout/src/liballoc/vec.rs:1555:10
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:397
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:572
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::panicking::panic_bounds_check
             at /checkout/src/libcore/panicking.rs:58
  10: <alloc::vec::Vec<T> as core::ops::index::Index<usize>>::index
             at /checkout/src/liballoc/vec.rs:1555
  11: panic::main
             at src/main.rs:4
  12: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:99
  13: std::rt::lang_start
             at /checkout/src/libstd/panicking.rs:459
             at /checkout/src/libstd/panic.rs:361
             at /checkout/src/libstd/rt.rs:61
  14: main
  15: __libc_start_main
  16: <unknown>
```

## Recoverable Errors with `Result`

Important Definition:
```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Real Example:

```
let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
```

The generic parameter are defined by `File::open` and in this case they are: `std::fs::File` is a File Handle, and `std::io::Error` is an error value.

In the `Ok`-case we can work with our file, in error case we panic! but provide an helpful error-message!

Example Output for File missing:
```
thread 'main' panicked at 'There was a problem opening the file: Error { repr: Os { code: 2, message: "No such file or directory" } }', src/main.rs:9:12
```

///Kandidat zum Löschen
### Match different errors:

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}
```

or

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}
```
//Kandiat

Maybe match of "error"

### Shortcut for panic:
expect mit msg.

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

### Propagating Error:

Das man jetzt in der jeder Methode auf das das result matchen müsste, könnte ziemlich ausarten. Deswegen Elvis to the rescue!

```
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
Der Elvis Operator sorgt dafür dass bei einem Ok, f befüllt wird, und bei einem Error die Funktion/Methode mit selbigen verlassen wird.

Wichtig ist, dass der Elvis Operator nur in Verbindung mit dem Return-Type Result<T,E> verwendet werden kann!

## To `panic!` or Not to `panic!`

Von einem panic gibt es keinen Weg zurück! Dann ist das Programm erstmal im Arsch. Bei einem Resut, gibt man dem Aufrufer der Methode aber eine Möglichkeit auf einen Fehler zu reagieren. Deswegen ist das Result erstmal eine gute default Wahl!

Für Prototyping and Tests bietet sich, unwrap und expect an. Beim Prototyping will man i.d.R auf eine robuste Fehlerbehandlung verzichten, sollte trotzdem der Fall auftreten bricht das Programm an klar definierten Stellen ab. Bei Tests will man eigentlich auch nicht dass der Test großartig weiterläuft man will sofort die Fehlerstelle!

An einigen Stellen weiß man auch meehr als der Compiler:
```
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```
Ich mein der String ist ganz klar eine Ip-Adresse hier können wir ohne Probleme auf unwrap zurückgreifen.
