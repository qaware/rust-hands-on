# Smart Pointer

## Einführung

Smart Pointer := Pointer mit zusätzlichen Meta-Daten und Fähigkeiten.

Smart Pointer sind in Rust aufgrund der strikten Einschränkungen bzgl. einfacher
Referenzen notwendig.

Beispiele:

- Datentypen, deren Größe zur Compile-Zeit nicht bekannt ist
- Große Datenmengen, deren Ownership übertragen werden soll
- "Interfaces": Wenn uns nur ein Trait eines Datenobjektes interessiert und
nicht der Inhalt
- Ownership, die erst zur Laufzeit aufgelöst werden kann, z.B. mehrere Owner

Implementierung durch Structs und Traits.

Im folgenden betrachten wir nur die zwei wichtigsten Beispiele,
`Box<T>` und `Rc<T>`.
Mehr Infos findet ihr im Rust Book und in der API-Dokumentation.

## `Box<T>`

`Box<T>` erlaubt es, Daten auf dem Heap anstatt auf dem Stack zu speichern.

Technisch ist `Box<T>` ein simpler pointer. Geht `Box<T>` out-of-scope,
wird das Objekt gelöscht. Wird `clone` auf `Box<T>` aufgerufen, wird das Objekt
kopiert.

Beispiel: Implementierung einer einfachen vorwärts-verketteten Liste:
[box/src/main.rs](box/src/main.rs).

## `Rc<T>`

`Rc<T>` erlaubt es, mehrere read-only Owner zu haben.

`Rc<T>` implementiert dazu einen reference-counting Smart Pointer,
bekannt und beliebt aus C++. Wird `clone` auf `Rc<T>` aufgerufen, wird der
counter erhöht. Geht `Rc<T>` out-of-scope, wird der counter erniedrigt. Erst
wennn er 0 erreicht, wird das Objekt gelöscht.

Beispiel: Implementierung einer einfachen vorwärts-verketteten Liste,
die mehrere Referenzen erlaubt: [rc/src/main.rs](rc/src/main.rs).
