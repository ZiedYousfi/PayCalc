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

    match get_number_of_hours(&input) {
        Ok(v) => {
            worked_hours += v;
        }
        Err(e) => {
            eprintln!("Couldn't parse per hour wage : {}", e)
        }
    }

    println!("Worked hours : {}", worked_hours);

    let mut to_pay: f64 = 0.0;

    let mut final_rate: f64 = per_hour;

    println!("To pay : {}", to_pay);
    println!("Final rate : {}", final_rate);

    for _ in 0..(worked_hours / increase_rate_in_hours) as usize {
        for _ in 0..increase_rate_in_hours as usize {
            to_pay += final_rate;
        }
        final_rate += increase_for_rate;
        println!("To pay : {}", to_pay);
        println!("Final rate : {}", final_rate);
    }

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

    to_pay -= already_paid;

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
                    println!("Found value: {}", value);
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value() {
        assert_eq!(get_value("*100").unwrap(), 100.0);
        assert_eq!(get_value("salary *50 per hour").unwrap(), 50.0);
        assert_eq!(get_value("rate is *75.5").unwrap(), 75.5);
        assert!(get_value("no asterisk here").is_err());
        assert!(get_value("* no number").is_err());
        assert!(get_value("").is_err());
    }

    #[test]
    fn test_get_number_of_hours() {
        assert_eq!(get_number_of_hours("*100 40").unwrap(), 40.0);
        assert_eq!(get_number_of_hours("hours: 8").unwrap(), 8.0);
        assert_eq!(get_number_of_hours("worked\t35\nhours").unwrap(), 35.0);
        assert_eq!(get_number_of_hours("10 20 30").unwrap(), 60.0);
        assert!(get_number_of_hours("no numbers").is_err());
        assert!(get_number_of_hours("").is_err());
    }

    #[test]
    fn test_separator_macro() {
        let separators = separator!();
        assert!(separators.contains(&" "));
        assert!(separators.contains(&"\t"));
        assert!(separators.contains(&"\n"));
        assert!(separators.contains(&"\r"));
        assert!(separators.contains(&"\x0B"));
        assert!(separators.contains(&"\x0C"));
    }
}
