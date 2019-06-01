# Tests
So wir haben jetzt schon eine recht ordentliche Applikation am Start unser Kunde ist glücklich alle Anforderungen sind abgedeckt, aber unser Kopf lässt uns nicht in Ruhe, was ist wenn wir doch einen Fehler gemacht haben? Immerhin haben wir keine Tests geschrieben! Ja aber wie schreiben wir die eigentlich?

Aber first things first, aufräumen.
> den Important kram löschen

Tests ganz einfach mit `cargo test`

```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Cool, erstmal wir haben Unterstützung von Unit-Tests direkt in Cargo eingebaut. Wir erhalten eine detailierte Nachricht zu den gelaufenen Tests und ob diese erfolgreich, fehlgeschlagen oder ignoriert wurden. Ferner gibt uns Cargo die anzahl an Benchmark-Tests welche allerdings nur mit nightly-build funktionieren. und mit `filtered out` sind Unit-Tests gemeint die durch die Kommandozeile ausgefiltert wurden, gleich dazu mehr.

Aber wie schreibe ich jetzt eigentlich einen Unit-Test? Am einfachsten Direkt in die `main.rs`

```
#[cfg(test)] //Sagt dem Compiler das der nachfolgende Abschnitt nicht teil des Release/Debug Binaries ist.
mod tests { //Hier deklarieren wir Code welcher in seinem eigenen "Package" lebt.
    use super::*; //Weil hier in anderem Package leben, müssen wir das Eltern-Package importieren.

    #[test] //Die nachfolgende Methode ist ein Unit-Test, wir könnten Helfer-Methoden ohne test-annotation in das package aufnehmen.
    fn hello_test() {
        let list = vec![Measure{time: 10, value: 1100}, //given
                Measure{time: 25, value: 2000},
                Measure{time: 33, value: 1000},
                Measure{time: 41, value: 1500}];

        let result = find_max(&list); //when

        assert_eq!(result.value, 2000); //then, mit Assert-Methode.
    }
}
```

Führen wir jetzt `cargo test` sehen wir einmal den Funktions-Namen und natürlich ob er funktioniert hat.

Was ist wenn ein Unit-Test fehlschlägt:

```
    #[test]
    fn failing_test() {
        let list: Vec<Measure<i32>> = Vec::new();

        find_max(&list);
    }
```

`cargo test` -> Jetzt sehen wir natürlich einen Unit-Test mehr, und der steht auf failed, und wir erhalten zusätzlich noch den std-out des fehlgeschlagenen Tests. der hilft aber nicht wirklich..

Den Backtrace trick in dem Fall kennen wir ja schon.

*Generell* können wir auch mit cargo viele unterschiedliche Parameter ausführen, die würde ich allerdings aus Zeitgründen mal weglassen.

Unabhängig davon: Richtig in unserem Fall ist natürlich, wenn wir die annotation mit #[should_panic] an die Methode kleben. Wie wir vorhin gesehen haben ist das aber nicht ganz optimal, weil wir nicht wirklich einschränken können wo die Panic auftritt und es zu false-positive kommen kann.

Bei den asserts hingegen.

> Bei dem Unit-Test den Wert ändern + `cargo test`

Erhalten wir wenigstens eine krude Fehlermeldung. Wir können, für eine verbesserte Lesbarkeit können wir auch die Fehlermeldung direkt setzen.

> assert um string erweitern + `cargo test`

Falls wirklich noch Zeit ist.
> Vielleicht ist jemanden schon aufgefallen, wir hatten doch ursprünglich gesagt, mit einem `panic` geht das Programm sofort kaputt. Wie wir hier sehen stimmt das nicht ganz, Rust ist schon noch in der Lage den Stacktrace aufzubauen, ferner gibt es Frameworks die helfen den Zustand vor dem Fehlerfall für Fehlerberichte abzusichern.

Natürlich können wir Unit-Tests auch ignorieren
```
#[ignore]
```
> `cargo test` auf das ignore hinweisen, und rükcgängig machen.

Und auch bei Unit-tests können wir mit Results arbeiten:

```
#[test]
    fn hello_test() -> Result<(), String> {
        let list = vec![Measure{time: 10, value: 1100},
                Measure{time: 25, value: 2000},
                Measure{time: 33, value: 1000},
                Measure{time: 41, value: 1500}];

        let result = find_max(&list);

//        assert_eq!(result.value, 2000, "find_max result passt net");
        if  result.value == 2000 {
            Ok(())
        } else {
            Err(String::from("find_max result passt net"))
        }
    }
```

Man kann auch, wie sie in Rust heißen IntegrationTests schreiben. Dazu müsste man im root-Ordner einfach einen Ordner hinzufügen und dort zusätzliche rs-files ablegen.

Im Gegensatz zu vohin müssen wir bei den Integration-Tests zuerst unser Programm importieren.

Bei den IntegrationTests gilt allerdings eine restriktion, diese sind nur für lib-Projekte möglich.

Diese Art von Projekten bieten einen nicht nur die IntegrationTests als Feature, sondern man kann mit ihnen auch Doc-Tests Definieren

```
/// Summary
///
/// detailed documentation
///
/// ```
/// let list = vec![Measure{time: 10, value: 1100},
///     Measure{time: 25, value: 2000},
///     Measure{time: 33, value: 1000},
///     Measure{time: 41, value: 1500}];
///
/// let result = find_max(&list);
/// assert_eq!(result.value,2000, "find_max result passt net");
/// ```
```
D.h. Rust passt gleich bei dem `cargo test` auf ob euere Documentation veraltet ist. zusätzlich dazu bietet Rust die Möglichkeit ein statisches Html euerer Lib an, die Std ist ebenfalls damit generiert worden.

Dass das nur mit libs geht, ist zwar doof, aber verschmerzbar. Das führt zu kleineren Binary-Projekten die relativ schnell in eine Lib abspringen...

> Nach Argumentation von Rust, lebt ein Binary von der Ausführung und eine Library davon dass sie von aussen verwendet mit vielen unterschiedlichen Aufrufen etc.

Kurze Verschnaufpause, was haben wir gelernt?

Wir wissen wie man mit cargo Tests ausführt und wir wissen wie man Unit-Tests schreibt, wir wissen auch dass man für Libraries IntegrationTests und Documentation-Test schreiben kann.
