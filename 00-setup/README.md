# 0. Setup
## Linux
1. Über Package-Manager `cargo` und die `rust-stdlib` installieren
   - Fedora:
   ```bash
   sudo dnf install cargo rust-src rustfmt
   ```
   - Ubuntu:
   ```bash
   sudo apt-get install cargo rust-src
   cargo install rustfmt   
   ```
   - Alternative zu package manager: rustup. 
   ```bash
   curl https://sh.rustup.rs -sSf | sh
   rustup component add rustfmt
   ```
2. In IntelliJ `rust` plugin installieren
3. Plugin konfigurieren: `Languages & Frameworks` -> `Rust`
   - Toolchain location: `/usr/bin`
   - Standard library: `/usr/lib/rustlib/src/rust/src`