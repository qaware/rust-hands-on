use std::io::Error;

struct Home {
    thermometer_living_room: Thermometer,
    thermometer_kitchen: Thermometer,

    barometer: Barometer,
}

trait Summary {
    fn summarize(&self) -> String {
        String::from("Have a nice day!")
    }
}

struct Thermometer {
    time_series_in_celsius: Vec<Measure<i32>>,
}

impl Summary for Thermometer {
    fn summarize(&self) -> String {
        format!("Temparature Current: {}", self.time_series_in_celsius.last().unwrap().value)
    }
}

struct Barometer {
    time_series_in_pascal: Vec<Measure<u32>>,
}

impl Summary for Barometer {
    fn summarize(&self) -> String {
        format!("Air-Pressure Current: {}", self.time_series_in_pascal.last().unwrap().value)
    }
}

struct Measure<T> {
    time: u64,
    value: T,
}

fn main() -> Result<(), Error> {
    let home = init()?;
    println!("-----");
    print_summary(&home);
    println!("-----");
    print_mips(&home);
    println!("-----");
    Ok(())
}

fn print_summary(home: &Home) {
    print_summary_from(&home.thermometer_living_room);
    print_summary_from(&home.barometer);
}

fn print_summary_from<T>(system: &T)
    where T: Summary
{
    println!("{}", system.summarize());
}

fn print_mips(home: &Home) {
    let max_temperature = find_max(&home.thermometer_living_room.time_series_in_celsius);
    println!("Temparature Living Room Max: Measured at {} - {} C", max_temperature.time, max_temperature.value);

    let max_temperature = find_max(&home.thermometer_kitchen.time_series_in_celsius);
    println!("Temparature Kitchen Max: Measured at {} - {} C", max_temperature.time, max_temperature.value);

    let overall_max = find_ultimate_max(
        &home.thermometer_kitchen.time_series_in_celsius,
        &home.thermometer_living_room.time_series_in_celsius,
    );
    println!("Temparature Overall Max: Measured at {} - {} C", overall_max.time, overall_max.value);

    let max_air_pressure = find_max(&home.barometer.time_series_in_pascal);
    println!("Air-Pressure Max: Measured at {} - {} Pa", max_air_pressure.time, max_air_pressure.value);
}

fn find_max<T>(list: &[Measure<T>]) -> &Measure<T>
    where T: std::cmp::PartialOrd
{
    let mut max = &list[0];

    for item in list.iter() {
        if item.value > max.value {
            max = item;
        }
    }

    max
}

fn find_ultimate_max<'a, T: std::cmp::PartialOrd>(list: &'a [Measure<T>], other: &'a [Measure<T>]) -> &'a Measure<T>
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

fn init() -> Result<Home, Error> {
    let thermometer_living_room = Thermometer {
        time_series_in_celsius: vec![
            Measure { time: 10, value: -5},
            Measure {time: 25, value: 2 },
            Measure { time: 33, value: 10 },
            Measure { time: 40, value: 8 }],
    };

    let thermometer_kitchen = Thermometer {
        time_series_in_celsius: vec![
            Measure { time: 10, value: 10 },
            Measure { time: 25, value: 11 },
            Measure { time: 33, value: 20 },
            Measure { time: 40, value: 18 }],
    };

    let barometer = Barometer {
        time_series_in_pascal: vec![
            Measure { time: 10, value: 1100 },
            Measure { time: 25, value: 2000 },
            Measure { time: 33, value: 1000 },
            Measure { time: 41, value: 1500 }],
    };

    Ok(Home {
        thermometer_living_room,
        thermometer_kitchen,
        barometer,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_test() -> Result<(), String> {
        let list = vec![Measure { time: 10, value: 1100 },
                        Measure { time: 25, value: 2000 },
                        Measure { time: 33, value: 1000 },
                        Measure { time: 41, value: 1500 }];

        let result = find_max(&list);

//        assert_eq!(result.value, 2000, "find_max result passt net");
        if result.value == 2000 {
            Ok(())
        } else {
            Err(String::from("find_max result passt net"))
        }
    }

    #[test]
    #[should_panic]
    fn failing_test() {
        let list: Vec<Measure<i32>> = Vec::new();

        find_max(&list);
    }
}
