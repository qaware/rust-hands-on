# Echo/SMTP Client

Client that sends smtp request (without checking for correctness of answers) and can be used both as client for echo and smtp servers. Usage:
```bash
cargo run -- <ADDRESS>
```
where `<ADDRESS>` usually consists of `<HOST>:<PORT>`, e.g. `localhost:9999`.