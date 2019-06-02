# 0. Setup

## Installation von Rust

### Mit Rustup (Alle Betriebssysteme)

Der Anleitung auf [rustup.rs](https://rustup.rs/) folgen (abhängig vom Betriebssystem). Dann:

```bash
rustup component add rustfmt
rustup component add rust-src
```

### Über Paketmanager (Linux)

1. Über Package-Manager `cargo` und `rust-src` installieren, `rustfmt` über package-manager oder cargo hinzufügen:

   - Fedora:
       ```bash
       sudo dnf install cargo rust-src rustfmt
       ```
   - Ubuntu:
       ```bash
       sudo apt-get install cargo rust-src
       cargo install rustfmt
       ```

## IntelliJ einrichten

1. In IntelliJ `rust` plugin installieren
2. Plugin konfigurieren (bei Installation ohne rustup): `Languages & Frameworks` -> `Rust`
   - Toolchain location: `/usr/bin`
   - Standard library: `/usr/lib/rustlib/src/rust/src`

## VSCode einrichten (optional)

Rust (rls) extension installieren.

![VSSCode screenshot with the Rust extension](rust-vscode-screenshot.png)