# Aufgaben
## Echo-server
Schreibt einen simplen Server, der eingehende Verbindungen akzeptiert, und nach jeder gelesenen Zeile eine Zeile mit dem selben Inhalt zurück sendet.

Für den Server gibt es ein ein [Grundgerüst](server-template), was euch das lästige connection aufbauen/thread starten abnimmt.

Einen [Beispiel-Client](client/) gibt es auch, ausführen könnt ihr ihn mit:
```bash
cargo run -- localhost:port
```

Falls ihr früher fertig seid, gibt es ein paar Anregungen für Erweiterungen:
 - geteilte message-history
 - unit-tests

Hilfreiche Doku:
 - [String](https://doc.rust-lang.org/std/string/struct.String.html)
 - [TcpStream](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
 - [Read](https://doc.rust-lang.org/std/io/trait.Read.html) / [BufRead](https://doc.rust-lang.org/std/io/trait.BufRead.html) / [Write](https://doc.rust-lang.org/std/io/trait.Write.html)
 - [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)
 - [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
 - [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
 - [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
 
Mit dem gegebenen Grundgerüst ist sollte die Aufgabe nicht zu schwer sein - die [Beispiellösung](solution/echo-server) implementiert alle Erweiterungen und ist daher etwas komplexer.


## SMTP-server
Wir erweitern unseren echo-server zu einem SMTP-server.
