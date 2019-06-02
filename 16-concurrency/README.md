# Concurrency

Die Rust Standard-Library bietet nur einfache parallele Konstrukte, die dafür
ein hohes Maß an Sicherheit garantieren. Im folgenden werden nur Threads
und Channels aus der Standard-Library vorgestellt.
Es gibt sehr gute Crates etwa für MPMC-Channels oder Thread Pools.

## Threads

Die Rust Standard-Library kennt nur einfache Threads. Jeder Rust-Thread
entspricht einem Thread im Betriebssystem.

Beispiel: [threads/src/main.rs](threads/src/main.rs)

## Channels

Auf bei Channels bietet die Standard-Library nur wenig: Eine Implementierung
von Multi-Producer Single-Consumer Channels.

Beispiel: [channels/src/main.rs](channels/src/main.rs)
