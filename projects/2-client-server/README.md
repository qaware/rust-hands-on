# Aufgaben
## Palindrom

## Echo-server
Schreibt einen Server, der eingehende Verbindungen akzeptiert, und (vielleicht in einem eigenen Thread?) nach jeder gelesenen Zeile eine Zeile mit dem selben Inhalt zurück sendet.
Einen Beispiel-Client dafür findet ihr unter `/client`, ausführen könnt ihr ihn mit:
```bash
cargo run -- localhost:port
``` 
## SMTP-server
Wir erweitern unseren echo-server zu einem SMTP-server.
Wenn ihr nicht so weit gekommen seit oder lieber auf einem sauberen Stand anfangen wollt, könnt ihr auf der Referenzimplementierung unter `echo-server/` aufbauen.