#[derive(Debug)]
pub enum ParseInputError {
    MissingAsterisk,
    InvalidNumber,
    NoHoursFound,
}

impl std::fmt::Display for ParseInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseInputError::MissingAsterisk => {
                write!(f, "Aucun astérisque (*) trouvé dans l'entrée")
            }
            ParseInputError::InvalidNumber => write!(f, "Format de nombre invalide"),
            ParseInputError::NoHoursFound => write!(f, "Aucune durée trouvée dans l'entrée"),
        }
    }
}

const SEPARATORS: [&str; 6] = [" ", "\t", "\n", "\r", "\x0B", "\x0C"];

pub fn calculate_payment(
    per_hour: f64,
    worked_hours: f32,
    increase_for_rate: f64,
    increase_rate_in_hours: f32,
) -> f64 {
    let mut to_pay = 0.0;
    let mut final_rate = per_hour;
    let mut hours_left = worked_hours;

    println!("\n🔧 Calcul du paiement en cours...");

    let full_periods = (worked_hours / increase_rate_in_hours).floor() as usize;
    for period in 0..full_periods {
        let segment = increase_rate_in_hours as f64 * final_rate;
        to_pay += segment;
        hours_left -= increase_rate_in_hours;

        println!(
            "➤ Période {} : {:.2}€ ({}h à {:.2}€/h)",
            period + 1,
            segment,
            increase_rate_in_hours,
            final_rate
        );
        final_rate += increase_for_rate;
    }

    if hours_left > 0.0 {
        let segment = hours_left as f64 * final_rate;
        println!("➤ Reste : {segment:.2}€ ({hours_left:.2}h à {final_rate:.2}€/h)",);
        to_pay += segment;
    }

    println!("✅ Paiement total : {to_pay:.2}€");
    to_pay
}

pub fn get_value(string: &str) -> Result<f64, ParseInputError> {
    let chars: Vec<char> = string.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c == '*' {
            let start = i + 1;
            let mut end = start;

            while end < chars.len()
                && (chars[end].is_ascii_digit() || chars[end] == '.' || chars[end] == ',')
            {
                end += 1;
            }

            if end > start {
                let value = string[start..end].replace(',', ".");
                return value
                    .parse::<f64>()
                    .map_err(|_| ParseInputError::InvalidNumber);
            }
        }
    }

    Err(ParseInputError::MissingAsterisk)
}

pub fn get_number_of_hours(string: &str) -> Result<f32, ParseInputError> {
    let chars: Vec<char> = string.chars().collect();
    let mut values: Vec<f32> = Vec::new();

    for (i, &c) in chars.iter().enumerate() {
        if SEPARATORS.contains(&c.to_string().as_str()) {
            let start = i + 1;
            let mut end = start;

            while end < chars.len()
                && (chars[end].is_ascii_digit() || chars[end] == '.' || chars[end] == ',')
            {
                end += 1;
            }

            if end > start {
                let value = string[start..end].replace(',', ".");
                if let Ok(num) = value.parse::<f32>() {
                    values.push(num);
                }
            }
        }
    }

    if values.is_empty() {
        return Err(ParseInputError::NoHoursFound);
    }

    Ok(values.iter().sum())
}
