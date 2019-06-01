# Einführung
Bis jetzt haben primitive Datentypen, Funktionen, Structs und Enums kennengelernt, d.h. wir Hauptsächlich auf dem Level von `C` programmiert.

Aber Rust bietet auch höhere Sprachkonzepte an, wie Generics die wir auch Teilweise angeschnitten haben `Result`, `Option` oder `Vec`. Genau dieses Sprachkonzept schauen wir uns jetzt genauer an.

Als Basis dazu habe ich uns ein kleines Programm geschrieben. Es handelt sich um ein "Programm welches im Embedded-System" Kontext ausgeführt werden soll. Dem Programm werden Messreihen eines Thermostates und eines Barometers übergeben und auf Basis der Daten sollen dem Nutzer wichtige Textausgaben übergeben werden.

> Programm von oben nach unten erklären

# Generics

Auch in unserem jetzigen Programm gibt es bereits ein Typen welcher mit generischen Typparametern arbeitet: `Result<T,E>`.

Ich würde vorschlagen das wir als erstes unsere Messpunkte vereinheitlichen:

```
struct Measure<T> {
    time: u64,
    value: T,
}
```
Natürlich könnten wir auch für den Zeitpunkt einen generischen Typparameter definieren (mit `,` separiert):

```
struct Measure<T, U> {
    time: U,
    value: T,
}
```

Aber wir wollen es in dem Programm nicht unnötig verkomplizieren also belassen wir es bei einem generischen Typparameter.

Definieren eines generischen Methode:

```
impl<T> Measure<T> {
    fn get_value(&self) -> &T {
        &self.value //; can be ommitet
    }
}
```
Ganz wichtig in dem impl block definieren wir für welchen Datentyp wir etwas definieren, denkbar wäre auch folgende Implementierung:

```
impl Measure<i32> {
  fn difference(&self, other: &Measure<i32>) -> i32 {
    (self.value - other.value).abs()
  }
}
```

Jetzt implementieren wir nicht mehr für den generischen Typparameter, sondern nur für `i32`.

Soviel zur Syntax implementieren wir unser Programm doch mal durch.

> `AirPressureMeasure` und `TemperatureMeasure` durch `Measure<T>` ersetzen.

Das ist schon mal ein richtiger Schritt in die richtige Richtung statt zwei Zeitreihen-Typen haben wir nun einen generischen. Aber Das Programm enthält immernoch viel zuviel doppelten code?

Wir verfahren wir jetzt weiter?

# Traits

Mit Traits, diese abstrahieren das verhalten von einem Datentypen. In Rust entspricht das implementieren eines Traits, der Aussage, dass ein Datentype eine bestimmte Funktionalität besitzt. In Java würde es dem entsprechen, wenn eine Klasse mehrere `Interface`s implementieren.

Als Erstes würde ich vorschlagen, wir vereinfachen die `print_summary` Methode in dem wir ein `Summary` Trait definieren:

```
trait Summary {
    fn summarize(&self) -> String;
}
```

Mit einer Default Imiplementieren:

```
trait Summary {
    fn summarize(&self) -> String {
        String::from("Have a nice day!")
    }
}
```

Jetzt müssen wir den Trait noch für unser `Thermometer` und `Barometer` implementieren.

```
impl Summary for Thermometer {
    fn summarize(&self) -> String {
        format!("Temparature Current: {}", self.time_series_in_celsius.last().unwrap().value)
    }
}
```
und
```
impl Summary for Barometer {
    fn summarize(&self) -> String {
        format!("Air-Pressure Current: {}", self.time_series_in_pascal.last().unwrap().value)
    }
}
```
Jetzt ist die print_summary methode schon deutlilch vereinfacht, die Logik hat sich in unserem Fall einfach nur verschoben, weil die Datentypen sehr Ähnlich sind, aber betrachten wir den Fall dass wir ein neues IOT-Gerät kaufen eine Freisprechanlage, mit den folgenden Daten:

```
struct DoorSpeakingSystem {
    recording: String,
}

impl Summary for DoorSpeakingSystem {
    fn summarize(&self) -> String {
        self.recording.clone()
    }
}
```

Damit will ich klarstellen, dass Traits unabhängig von dem Datentypen sind, sie sagen lediglich aus, dass ein Datentyp eine gewisse Funktionalität hat!

> `DoorSpeakingSystem` wieder löschen

*Vorsicht* in unserem Fall ist der Struct und der Trait im selben Scope. Wollen wir einen fremden Trait benutzen müssen wir diesen erst importieren.

Ferner ist die orphane rule zu beachten. Wir können keine externen Traits für externe Typen implementieren. Summary und Barometer sind locale Typen, wir können auch `Display` für den lokalen Typ `Barometer` implementieren. Wir können aber den `Display` Trait nicht für den `Vec<>` oder den `Summary` implementieren. This restriction is part of a property of programs called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa.

Wir können aber auch Traits in Methodenparametern verwenden:

```
fn print_summary(home: &Home) {
    print_summary_for(&home.thermometer);
    print_summary_for(&home.barometer);
}

fn print_summary_for(system: &impl Summary) {
    println!("{}", system.summarize());
}
```

Alternative Schreibweisen:

```
fn print_summary_for<T: Summary>(system: &T) {
    println!("{}", system.summarize());
}
```

```
fn print_summary_for<T>(system: &T)
    where T: Summary
{
    println!("{}", system.summarize());
}
```

Wenn der Typparameter mehr wie einen Trait implementieren muss, werden die einzelnen Traits mit `+` aufgezählt:
```
fn print_summary_for<T>(system: &T)
    where T: Summary + Display
{
    println!("Given System: {}"", system);
    println!("{}", system.summarize());
}
```
Bei `Display` handelt es sich um einen Standard-Library Trait. Den erst über use importieren müsste und Implementieren müsste. Weil ich das jetzt nicht tun werde, werde ich jetzt wieder entfernen.

Natürlich können wir einen Trait auch als Rückgabeparameter definieren.

```
fn example() -> impl Summary {
    Barometer {time_series_in_pascal: Vec::new()}
}
```

Aber wir müssen darauf achten, dass der zurück gegebene Trait eindeutig ist:

```
fn example(switch: bool) -> impl Summary {
    if switch {
        Barometer {time_series_in_pascal: Vec::new()}
    } else {
        Thermometer {time_series_in_celsius: Vec::new()}
    }
}
```
Da Rust die genaue Größe benötigt um die Daten auf den Stack abzulegen, deswegen ist das obere Konstrukt so nicht möglich. Theoretisch wäre dieses Konstrukt möglich, dazu müssten wir aber den Trait erst verpacken, damit er wieder eine feste Größe hat. Die dazu benötigten Sprachkonstrukte werden euch später gezeigt, beziehungsweise wird es hier schnell kompliziert.

> `fn example` wieder löschen

So wollen wir uns noch einen unschönen Part unseres Programes annehmen: die `find_max` methoden. Hier müssen uns doch die generischen Typparameter helfen!

Fahren wir erstmal einen naiven Ansatz:

```
fn find_max<T>(list: &[Measure<T>]) -> &Measure<T> {
    let mut max = &list[0];

    for item in list.iter() {
        if item.value > max.value {
            max = item;
        }
    }

    max
}
```

Bei dem Compilieren sagt uns rust jetzt schon, damit das hier funktioniert benötigen wir aus der Std einen Trait `std::cmp::PartialOrd` das ist der Trait für das `>`. Ganz Wichtig `i32` und `u32` implementieren diesen Trait, wenn ich diesen Trait für `Measure` implementiert hätte würde es auch ohne das `.value` gehen.

> PartialOrd einfügen

Danach funktioniert unser Programm wieder. Es gibt eine ganze Menge von Standard-Lib Traits die das Programmieren in Rust unglaublich vereinfachen. Aber fassen wir nochmal zusammen was wir gelernt haben:

Wir wissen nun wie man generische Typparameter für ein Struct definiert und wissen wie diese eingeschränkt werden. Wir haben Traits kennengelernt, wie man diese definiert und für ein struct implementiert und verwendet.


# Lifetimes

Rust basiert auf drei Schlüsselkonzepte *Ownership*, *Borrowing* und *Lifetimes*. Die ersten beiden haben wir schon kennengelernt und jetzt lernen wir das letzte Konzept kennen: *Lifetimes*. Bisher haben wir uns durch geeignete Programmierweise um das Thema rumgeschifft, aber auf Dauer ist es natürlich unvermeidbar.

Was sind Lifetimes?

> Die alte `main` auskommentieren

```
fn main() {
    let r;

    {
        let x = 5;

        r = &x;
    }

    println!("{}", r);
}
```

Fehlermeldung zeigt uns das borrwing an, und bis wohin der Wert lebt und wo er benötigt wird.

zeichnen wir die Lebenszeit der einzelnen Variablen mal explizit auf.

```
fn main() {
    let r;              //------+ 'b
                        //      |
    {                   //      |
        let x = 5;      //-+ 'a |
                        // |    |
        r = &x;         //-+    |
    }                   //      |
                        //      |
    println!("{}", r);  //------+
}
```

Mit dem verlassen des Scopes wird, wie wir hier sehen, der Wert der Variable x gelöscht und lebt bei dem aufruf des `println` nicht mehr!

Fixen können wir das problem nur wenn wir dafür sorgen dass die Lebenszeiten passen:

```
fn main() {
    let r;              //------+ 'b
                        //      |
                        //      |
        let x = 5;      //-+ 'a |
                        // |    |
        r = &x;         // |    |
                        // |    |
                        // |    |
    println!("{}", r);  //-+----+
}
```

Jetzt lebt `x` genau so lange wie `r`. Wie wir an diesem einfachen Beispiel sehen, orientiert sich die `Lifetime` am Stack.

*Hinweise Syntax und Regeln*

So wie ich das jetzt schon geschrieben habe ist auch die syntax von Lifetimes, sehen wir uns das folgende Beispiel an:

```
fn main() {
    let a = 5;
    let b = 10;

    foo(&a, &b);
}

fn foo(a : &i32, b: &i32) {
    println!("{}, {}", a, b)
}
```

`&i32` als Reference ohne explizite Lifetime-Angabe, `&'a i32` als Referenz mit expliziter Angabe.
und `&'a mut i32` als mutable Reference

In vielen Fällen muss man keine explizite Angabe von Lifetimes machen unser aktuelles Beispiel kommt ebenfalls ohne diese aus. Rust definiert sich anhand von den folgenden Regeln einfach eigene Lifetimes. Sollte ein Problem mit den Lifetimes zum Compilezeitpunkt auftreten führt das naürlich zu einen Fehler, um den sich der Programmierer kümmern muss. Die Regeln lautern folgendermaßen:

* Jeder Eingabe Parameter mit einer Referenz erhält seine eigene Lifetime
* Gibt es bloß einen Eingabe Lifetime-Parameter entspricht der Ausgabe-Parameter diesem
* Is einer der Eingabe Parameter `self` oder `&mut self` wird die ausgabe-lebenszeit an `self` eingebunden

Bei vielen fällen wie z.B. `fn foo(s: str) -> &str` klappt dass ganz gut, aber wie gesagt, findet der Compiler nach der Prüfung noch "lücken" oder es gibt Probleme wird der Vorgang mit einem Fehler abgebrochen.

Erweitern wir mal unser Beispiel:

```
fn foo(a : &i32, b: &i32) -> &i32 {
    if a > b {
        a
    } else {
        b
    }
}
```

`println!("{}", foo(&a, &b));`

> Cargo run führt zu `expected lifetime parameter`.

Aus Sicht von Rust, nach Anwendung der genannten Regeln sieht `foo` folgendermaßen aus:
```
fn foo<'a, 'b>(a : &'a i32, b: &'b i32) -> &?i32 {
  ...
```
Der Compiler könnte einfach eine Lifetime erfinden, die würde allerdings mit dem verlassen des Scopes der Methode enden, deswegen macht ein neuer Scope keinen Sinn. Es muss einer der beiden Eingabe lifetimes sein, aber den richtigen kann der RustCompiler leider nicht feststellen.

Wir müssen also Rust helfen:
```
fn foo<'a>(a : &'a i32, b: &'a i32) -> &'a i32 {
```

Ändern wir jetzt wieder die Lebenszeit der Variablen:
```
fn main() {
    let a = 5;
    let b = 10;

    let result = foo(&a, &b);
    println!("{}", result);
}
```

Funktioniert noch:
```
fn main() {
    let a = 5;
    let b = 10;

    let result;
    {
        result = foo(&a, &b);
    }
    println!("{}", result);
}
```

funktioniert auch noch:
```
fn main() {
    let a = 5;

    let result;
    {
        let b = 10;
        result = foo(&a, &b);
    }
    println!("{}", result);
}
```

funktioniert nicht mehr, weil b nicht lange genug lebt! Rust braucht die Lifetimes um zu evaluieren wie lange ein Parameter lebt und steigt aus, sobald wir eine Variable benutzen wollen die nicht so lange lebt wie wir es mind bräuchten.

Jetzt muss ich als Java-Programmierer etwas aufpassen, weil ich mir nicht nur vorher Überlegen muss wem die Variablen gehören, sondern auch wie lange diese Leben sollen! Als Anfänger wird man hier schnell in Probleme laufen, das wichtigste ist verstehen *Warum* es nicht funktioniert. Und vorallem am Anfang finde ich es in Ordnung, wenn man *nachdem man es verstanden hat* das Struct einfach zu `.clone()` es muss einfach klar sein, dass das andauernde kopieren von Speicher sich zu einem laufzeittechnisches Problem entwickeln kann, das Problem bei der Wurzel zu lösen ist natürlich der richtigere Weg, aber bevor man garnicht mehr weiter kommt und total die Motivation verliert, kann man sowas machen.

Machen wir zur Wiederholung nochmal ein Beispiel:

> Die Beispiel Blöcke entfernen

Ich mein, wer hat zuhause schon nur *ein* Thermometer wir brauchen mind. zwei Thermometer eines in der Küche und eines Wohnzimmer.

```
struct Home {
  thermometer_living_room: Thermometer,
  thermometer_kitchen: Thermometer,

  barometer: Barometer,
}
```
> In init entsprechend kopieren!

> print_mips erweiter um zusätzlichen max wert.

Wenn wir jetzt wieder die echte maximal temparatur haben wollen, könnten wir die folgende methode definieren:

```
fn find_ultimate_max<T: std::cmp::PartialOrd>(list: &[Measure<T>], other: &[Measure<T>]) -> &Measure<T>
{
    let mut max = &list[0];

    for item in list.iter() {
        if item.value > max.value {
            max = item;
        }
    }
    for item in other.iter() {
        if item.value > max.value {
            max = item;
        }
    }

    max
}
```

hier wird wieder Compiler meckern, wir müssen wieder den selben Trick anwenden:
```
fn find_ultimate_max<'a, T: std::cmp::PartialOrd>(list: &'a [Measure<T>], other: &'a [Measure<T>]) -> &'a Measure<T>
```

Was in Rust noch wichtig ist, dass man in Rust an das Struct selbst ebenfalls eine Lifetime kleben kann.

> main wieder auskommentieren

```
struct Important<'a> {
    answer: &'a str,
}

impl<'a> Important<'a> {
    fn answer(&self) -> &'a str {
        self.answer
    }

    fn real_answer(&self) -> i32 {
        42
    }
}

fn main() {
    let foo = "foo";
    let imp = Important {
        answer: foo,
    };

    println!("{}", imp.answer());
    println!("{}", imp.real_answer());
}
```

Es ist wichtig zu erwähnen, dass die Lifetime auf Struct-Ebene syntaktisch genau so behandelt werden wie ein Traits.

Ferner gibt es noch einen besondere Lifetime und zwar `'static`.

Der String in der Zeile `let foo = "foo";` kommt aus dem Daten-Segment eines Programmes und ist unveränderbar und lebt über die gesamte laufzeit des programmes. Es handelt sich also um den folgenden Datentypen:

`let foo: &'static str = "foo";`

