use std::{
    fmt::Error,
    io::{self},
};

macro_rules! separator {
    () => {
        [" ", "\t", "\n", "\r", "\x0B", "\x0C"]
    };
}

fn main() {
    println!("Hello, world!");

    let mut increase_for_rate: f64 = 0.0;
    let mut increase_rate_in_hours: f32 = 0.0;

    let mut rate_input = String::new();
    println!("Please enter the rate increase:");
    match io::stdin().read_line(&mut rate_input) {
        Ok(_) => match rate_input.trim().parse::<f64>() {
            Ok(val) => increase_for_rate = val,
            Err(e) => eprintln!("Failed to parse rate: {}", e),
        },
        Err(e) => eprintln!("Failed to read rate: {}", e),
    }

    let mut hours_input = String::new();
    println!("Please enter the hours for increase:");
    match io::stdin().read_line(&mut hours_input) {
        Ok(_) => match hours_input.trim().parse::<f32>() {
            Ok(val) => increase_rate_in_hours = val,
            Err(e) => eprintln!("Failed to parse hours: {}", e),
        },
        Err(e) => eprintln!("Failed to read hours: {}", e),
    }

    println!("Increase for rate: {}", increase_for_rate);
    println!("Increase rate in hours: {}", increase_rate_in_hours);
    println!(
        "Increase rate: {}",
        increase_for_rate / increase_rate_in_hours as f64
    );

    let mut input: String = String::new();
    let mut per_hour: f64 = 0.0;
    let mut worked_hours: f32 = 0.0;

    println!("Please enter the per hour wage (starting with \"*\") and hours worked in one line:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("You said : {}", input),
        Err(e) => {
            eprintln!("Couldn't get input {}", e)
        }
    }

    match get_value(&input) {
        Ok(v) => {
            per_hour += v;
        }
        Err(e) => {
            eprintln!("Couldn't parse per hour wage : {}", e)
        }
    }

    //assert!(per_hour != 0.0);
    println!("Per hour: {}", per_hour);

    match get_number_of_hours(&input) {
        Ok(v) => {
            worked_hours += v;
        }
        Err(e) => {
            eprintln!("Couldn't parse per hour wage : {}", e)
        }
    }

    //assert!(worked_hours != 0.0);

    println!("Worked hours : {}", worked_hours);

    let final_rate = per_hour
        + (worked_hours as f64 / increase_rate_in_hours as f64).floor() * increase_for_rate;

    println!("Final rate : {}", final_rate);

    let mut already_paid: f64 = 0.0;

    let mut already_paid_input: String = String::new();

    println!("Please enter the amount already paid:");
    match io::stdin().read_line(&mut already_paid_input) {
        Ok(_) => match already_paid_input.trim().parse::<f64>() {
            Ok(val) => {
                already_paid = val;
            }
            Err(e) => println!("Couldn't parse that : {}", e),
        },
        Err(e) => println!("Couldn't read that : {}", e),
    }

    let to_pay: f64 = worked_hours as f64 * final_rate - already_paid;

    println!("To pay : {}", to_pay);
    println!("Don't care about the currency, just pay me!");
    println!("Done :)");
}

/// This function takes a string and returns the value found after the '*' character.
/// If no number is found, it returns an error.
fn get_value(string: &str) -> Result<f64, Error> {
    let chars: Vec<char> = string.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c == '*' {
            let start = i + 1;
            let mut end = start;

            while end < chars.len() && chars[end].is_ascii_digit() {
                end += 1;
            }

            if end > start {
                if let Ok(value) = string[start..end].parse::<f64>() {
                    return Ok(value);
                }
            }
        }
    }

    Err(Error)
}

/// This function takes a string and returns the number of hours found after the separators (refer to the macro).
/// If no number is found, it returns an error.
fn get_number_of_hours(string: &str) -> Result<f32, Error> {
    let chars: Vec<char> = string.chars().collect();

    let mut values: Vec<f32> = Vec::new();

    for (i, &c) in chars.iter().enumerate() {
        if separator!().contains(&c.to_string().as_str()) {
            let start = i + 1;
            let mut end = start;

            while end < chars.len() && chars[end].is_ascii_digit() {
                end += 1;
            }

            if end > start {
                if let Ok(value) = string[start..end].parse::<f32>() {
                    values.push(value);
                }
            }
        }
    }

    if values.is_empty() {
        return Err(Error);
    }

    Ok(values.iter().sum())
}
