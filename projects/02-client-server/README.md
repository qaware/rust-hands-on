# Aufgaben
## Echo-server
Schreibt einen simplen Server, der eingehende Verbindungen akzeptiert, und nach jeder gelesenen Zeile eine Zeile mit dem selben Inhalt zurück sendet.

Für den Server gibt es ein ein [Grundgerüst](server-template/), was euch das lästige connection aufbauen/thread starten abnimmt.

Einen [Beispiel-Client](client/) gibt es auch, ausführen könnt ihr ihn mit:
```bash
cargo run -- localhost:port
```
Alternativ geht natürlich auch `netcat` oder ähnliches.

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
Da Echos auf Dauer etwas langweilig sind, erweitern wir unseren Server und implementieren wir einen Teil des SMTP-Protokolls.
Der Server soll in der Lage sein, Mails entgegenzunehmen und zu speichern.
Das Protokoll ist rein textbasiert und relativ simpel.
Ein Ablauf könnte zum Beispiel so aussehen:

```
<- 220 smtp.server.com Simple Mail Transfer Service Ready
-> HELO localhost
<- 250 Hello localhost
-> MAIL FROM:<user@localhost>
<- 250 OK
-> RCPT TO:<admin@localhost>
<- 250 OK
-> DATA
<- 354 Send message content
-> <Mail Data>
-> .
<- 250 OK
-> QUIT
<- 221 Bye
```

Das Ganze ist in [rfc821](https://tools.ietf.org/html/rfc821) spezifiziert, aus dem Protokoll sind dabei die Abschnitte **3.1 MAIL** und **3.5 OPENING AND CLOSING** relevant.

Zum testen könnt ihr wieder den [Beispiel-Client](client/) verwenden.
Oder ihr konfiguriert euren E-Mail Client, dass er Mails an euren SMTP-Server schickt - das geht z.B. in Mozilla Thunderbird.

Eine [Musterlösung](solution/smtp-server) gibt es auch wieder.