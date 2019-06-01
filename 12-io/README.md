# I/O

Und jetzt sehen wir uns einen weiteren wichtigen Teil einer Applikation an, I fucking O. Unserem Kunden ist aufgefallen, dass das Progrmam immernur den selben Output liefert, das ist schlecht also müssen wir uns nochmal an die Kiste hocken!

Einfachste IO mit `println!` und `io::stdin()`!

Ausgabe auf dem std-out ist ja schon drin für ein einfaches einlesen der Werte mittels stdin:

```
use std::io;
```
Erstmal das module imporiteren indem das stdin deklariert ist.

```
let mut input = String::new();
io::stdin().read_line(&mut input)?;

let splitted_input: Vec<&str> = input.split(' ').collect();
let measures:Vec<Measure<i32>> = splitted_input.chunks(2)
    .filter(|chunk| chunk[0].parse::<u64>().is_ok() && chunk[1].parse::<i32>().is_ok())
    .map(|chunk|

                        Measure {
                            time: chunk[0].parse().unwrap_or(0),
                            value: chunk[1].parse().unwrap_or(0),
                        }
                    ).collect();

let thermometer_living_room = Thermometer{
    time_series_in_celsius: measures
};
```

für std-err, könnten wir eprintln!("") benutzen.

Zum Einlesen einer Datei können wir:

`use std:fs;` Und

>let mut input und io::stdin auskommentieren

`let input = fs::read_to_string("./input")?;`

Konfiguriertbar und auslesen der Kommandozeilen Argumente ist mit
`use std:env;` und

```
let arguments: Vec<String> = env::args().collect();
let input = fs::read_to_string(&arguments[1])?;
```

Weitere I/O:

`env::var("BAROMETER_ENABLED").is_err();`

oder falls wir einen TCP-Client benötigten

`use std::net::{TcpStream}`
