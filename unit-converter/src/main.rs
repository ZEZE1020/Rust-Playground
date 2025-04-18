use std::env;
use unit_converter::units::{Length, convert_length};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <value> <from_unit> <to_unit>", args[0]);
        eprintln!("Supported units: m (meters), km (kilometers), mi (miles), ft (feet), in (inches)");
        return;
    }

    let value: f64 = match args[1].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: Invalid number");
            return;
        }
    };

    let from_unit = &args[2];
    let to_unit = &args[3];

    let length = match from_unit.to_lowercase().as_str() {
        "m" => Length::Meters(value),
        "km" => Length::Kilometers(value),
        "mi" => Length::Miles(value),
        "ft" => Length::Feet(value),
        "in" => Length::Inches(value),
        _ => {
            eprintln!("Error: Unsupported input unit");
            return;
        }
    };

    match convert_length(length, to_unit) {
        Ok(result) => println!("{} {} = {} {}", value, from_unit, result, to_unit),
        Err(e) => eprintln!("Error: {}", e),
    }
}