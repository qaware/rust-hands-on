#0. Setup
1. Über Package-Manager `cargo` und die `rust-stdlib` installieren
   - Fedora: `sudo dnf install cargo rust-std-static`
   - Ubuntu: ?
   - Sonstige: Nachschlagen! Alternative zu package manager: `rustup`
2. In IntelliJ `rust` plugin installieren
3. Plugin konfigurieren: `Languages & Frameworks` -> `Rust`
   - Toolchain location:
     - Fedora: `/usr/bin`
     - Ubuntu: ?
     - Sonstige: rausfinden über `which cargo`
   - Standard library:
     - Fedora: `/usr/lib/rustlib/src/rust/src`
     - Ubuntu: ?
     - Sonstige: ???