#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io;

fn main() {
    println!("Celcius <=> Farenheit temperature converter");

    lazy_static! {
        static ref TEMPERATURE_REGEXP: Regex = Regex::new(r"^(\-?\d+\.*\d*)([CF])$").unwrap();
    }

    loop {
        println!("Please enter the temperature to convert,\nfollowed by C or F (for Celcius or Farenheit)");

        let mut input_temperature = String::new();

        io::stdin().read_line(&mut input_temperature)
            .expect("Failed to read line");

        let input_temperature = input_temperature.trim();

        let is_input_valid = TEMPERATURE_REGEXP.is_match(&input_temperature);
        if !is_input_valid {
            println!("Invalid input given: {}", input_temperature);
            continue;
        }

        let cap = TEMPERATURE_REGEXP.captures(&input_temperature).unwrap();

        let temperature: f32 = match cap[1].trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let unit = &cap[2];

        let result_temperature = match unit {
            "C" => {
                let mut result = format!("{}", ((temperature * 1.8) + 32.0).to_string());
                result.push('F');
                result
            },
            "F" => {
                let mut result = format!("{}", ((temperature - 32.0) * 0.5556).to_string());
                result.push('C');
                result
            }
            _ => {
                continue
            }
        };

        println!("Converted temperature: {} -> {}", &input_temperature, &result_temperature);
    }
}
