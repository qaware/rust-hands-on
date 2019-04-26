# Error Handling

Es gibt in Rust zwei Fehlerfälle: Fehler, von denen man sich retten kann und panics. Ein panic ist final und kann nicht aufgehalten werden.

Es gibt keine Exceptions.

## Panic

```
fn main() {
  panic!("Frauen und Kinder zu erst!");
}
```

Ein panic kann durch das Panic-Macro `panic!` selbst ausgelöst werden.

Dieses Programm stirbt mit:

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'Frauen und Kinder zu erst!', src/main.rs:2:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Beispiel für ein panic, das nicht selbst ausgelöst wurde:

```
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

Das würde in C funktionieren und Speicher lesen, der nicht mehr in `v` ist. In Rust stirbt es:

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
99', /checkout/src/liballoc/vec.rs:1555:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Mehr Informationen gibt es mit `RUST_BACKTRACE=1`:

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

Im Prinzip sucht man nun in dem Stacktrace von unten nach oben solange, bis man ein Sourcefile findet, das man selbst geschrieben hat. In unserem Fall `11: panic::main at src/main.rs:4`.

## Fehler, von denen man sich retten kann

Wie funktionert das ohne Exceptions? Dafür gibt es, wer hätte es gedacht, ein Enum!

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Ok(T)` ist der Gutfall, `Err(E)` ist der Fehlerfall.

### Ein Beispiel mit `Result`

```
let f = File::open("hello.txt"); // f: Result<std::fs::File, std::io::Error>

let x = match f {
    Ok(file) => file, // file: std::fs::File
    Err(error) => { // error: std::io::Error
        panic!("There was a problem opening the file: {:?}", error)
    },
};

// use x
```

Im `Ok`-Fall können wir mit der Datei arbeiten. Im `Err`-Fall panicken wir, aber schreiben eine nette Fehlermeldung.

Beispielausgabe:
```
thread 'main' panicked at 'There was a problem opening the file: Error { repr: Os { code: 2, message: "No such file or directory" } }', src/main.rs:9:12
```

### Shortcut for panic:
`.expect` mit Fehlermeldung.

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Konnte hello.txt nicht öffnen");
}
```

`expect` gibt entweder `T` zurück, falls `Ok(T)` oder panic mit der Fehlermeldung, wenn `Err`.

Wenn man faul ist:
```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

Hier muss man sich klar sein, dass im Fehlerfall eine panic das Programm beendet. Sollte man nur in Tests und beim Prototypen verwenden, wir kommen am Ende des Kapitels nochmal drauf zu sprechen.

### Fehler weiterpropagieren:

Man müsste nun bei jeder Methode, die ein `Result` zurückgibt, sowas schreiben:

```
let x = match f {
    Ok(file) => file,
    Err(error) => return Result::Err(error),
};
```

Das artet ziemlich aus. Wer schon mal Go-Code gesehen hat, weiß, wovon ich rede. Rust hat dafür einen Operator namens `?`:

```
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // Beachte das ? am Ende
    let mut s = String::new();
    f.read_to_string(&mut s)?; // Beachte das ? am Ende
    Ok(s)
}
```
Der Operator sorgt dafür, dass bei einem `Ok` f befüllt wird, und bei einem Error die Funktion/Methode mit `Result::Err` verlassen wird.

Der Operator kann nur in Verbindung mit dem Return-Type `Result<T,E>` verwendet werden!

## To panic or not to panic?

Von einem panic gibt es keinen Weg zurück! Das Programm ist dann tot. Bei einem Result gibt man dem Aufrufer der Methode eine Möglichkeit, auf einen Fehler zu reagieren. Deswegen ist `Result` erstmal ne gute Wahl.

Sollte der Code in einen ungültigen Zustand kommen, sollte ein `panic` verwendet werden. Beispiel: Ungültige Eingabedaten, sich widersprechende Eingabedaten, fehlende Werte, UND wenn dieser ungültige Zustand nicht erwartet wird. `panic` wird verwendet,
wenn man das Programm nicht mehr sinnvoll retten kann. Das ist ungefähr wo wie ein `System.exit()` in Java.

Eine Nutzereingabe z.B. sollte man mit `Result` parsen, um dem Benutzer die Möglichkeit zu geben, sich zu korrigieren. Ungültige Eingabeparameter in eine Funktion, wie z.B. ein Index, der out of bound ist, wird meistens ein panic.

Für Prototyping and Tests bietet sich `unwrap` und `expect` an. Beim Prototyping will man in der Regel auf eine robuste Fehlerbehandlung verzichten. Sollte aber trotzdem der Fehlerfall auftreten, dann bricht das Programm an klar definierten Stellen ab. 

Bei Tests will man auch nicht, dass der Test weiterläuft, wenn man einen Fehler gefunden hat. Fail fast.

Manchmal benötigt man auch `.unwrap`, weil man schlauer ist als der Compiler (generell sollte man annehmen, dass der Compiler schlauer ist als man selbst, aber die Ausnahme bestätigt die Regel):

```
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```
"127.0.0.1" ist ganz sicher eine valide IP-Adresse, deswegen ist hier `.unwrap` unbedenklich.