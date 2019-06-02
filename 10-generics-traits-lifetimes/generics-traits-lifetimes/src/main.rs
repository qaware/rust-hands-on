use std::io::Error;

struct Home {
    thermometer: Thermometer,
    barometer: Barometer,
}

struct Thermometer {
    time_series_in_celsius: Vec<TemperatureMeasure>,
}

struct Barometer {
    time_series_in_pascal: Vec<AirPressureMeasure>,
}

struct TemperatureMeasure {
    time: u64,
    value: i32,
}

struct AirPressureMeasure {
    time: u64,
    value: u32,
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
    let temp_values = &home.thermometer.time_series_in_celsius;
    let latest_temp = temp_values.last().unwrap();
    println!("Temparature Current: {}", latest_temp.value);

    let air_pressure_values = &home.barometer.time_series_in_pascal;
    let latest_air_pressure = air_pressure_values.last().unwrap();
    println!("Air-Pressure Current: {}", latest_air_pressure.value);
}

fn print_mips(home: &Home) {
    let max_temperature = find_max_temperature(&home.thermometer.time_series_in_celsius);
    println!("Temparature Max: Measured at {} - {} C", max_temperature.time, max_temperature.value);

    let max_air_pressure = find_max_pressure(&home.barometer.time_series_in_pascal);
    println!("Air-Pressure Max: Measured at {} - {} Pa", max_air_pressure.time, max_air_pressure.value);
}

fn find_max_temperature(list: &[TemperatureMeasure]) -> &TemperatureMeasure {
    let mut max = &list[0];

    for item in list.iter() {
        if item.value > max.value {
            max = item;
        }
    }

    max
}

fn find_max_pressure(list: &[AirPressureMeasure]) -> &AirPressureMeasure {
    let mut max = &list[0];

    for item in list.iter() {
        if item.value > max.value {
            max = item;
        }
    }

    max
}

fn init() -> Result<Home, Error> {
    let thermometer = Thermometer {
        time_series_in_celsius: vec![
            TemperatureMeasure { time: 10, value: -5 },
            TemperatureMeasure { time: 25, value: 2 },
            TemperatureMeasure { time: 33, value: 10 },
            TemperatureMeasure { time: 40, value: 8 }],
    };

    let barometer = Barometer {
        time_series_in_pascal: vec![
            AirPressureMeasure { time: 10, value: 1100 },
            AirPressureMeasure { time: 25, value: 2000 },
            AirPressureMeasure { time: 33, value: 1000 },
            AirPressureMeasure { time: 41, value: 1500 }],
    };

    Ok(Home {
        thermometer,
        barometer,
    })
}
