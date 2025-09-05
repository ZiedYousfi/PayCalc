use std::io;

// Type d'erreur personnalisé
#[derive(Debug)]
pub enum ParseInputError {
    MissingAsterisk,
    InvalidNumber,
    NoHoursFound,
    IoError(String),
}

impl std::fmt::Display for ParseInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseInputError::MissingAsterisk => {
                write!(f, "Aucun astérisque (*) trouvé dans l'entrée")
            }
            ParseInputError::InvalidNumber => write!(f, "Format de nombre invalide"),
            ParseInputError::NoHoursFound => write!(f, "Aucune durée trouvée dans l'entrée"),
            ParseInputError::IoError(e) => write!(f, "Erreur d'entrée/sortie : {e}"),
        }
    }
}

const SEPARATORS: [&str; 6] = [" ", "\t", "\n", "\r", "\x0B", "\x0C"];

pub fn get_rate_increase() -> Result<f64, ParseInputError> {
    let mut input = String::new();
    println!("➡️  Entrez le montant de l'augmentation du taux horaire après chaque palier.");
    println!("   Exemple : 10 (ce qui signifie +10€/h après chaque palier)");

    io::stdin()
        .read_line(&mut input)
        .map_err(|e| ParseInputError::IoError(e.to_string()))?;

    input
        .trim()
        .replace(',', ".")
        .parse::<f64>()
        .map_err(|_| ParseInputError::InvalidNumber)
}

pub fn get_hours_for_increase() -> Result<f32, ParseInputError> {
    let mut input = String::new();
    println!("➡️  Entrez le nombre d'heures par palier avant l'augmentation du taux.");
    println!("   Exemple : 10 (le taux augmente toutes les 10h)");

    io::stdin()
        .read_line(&mut input)
        .map_err(|e| ParseInputError::IoError(e.to_string()))?;

    input
        .trim()
        .replace(',', ".")
        .parse::<f32>()
        .map_err(|_| ParseInputError::InvalidNumber)
}

pub fn get_wage_and_hours() -> Result<(f64, f32), ParseInputError> {
    let mut input = String::new();
    println!(
        "\n➡️  Entrez sur une seule ligne le taux horaire (commençant par '*') suivi du nombre d'heures travaillées."
    );
    println!("   Exemple : *50 40 (signifie 50€/h pendant 40 heures)");

    io::stdin()
        .read_line(&mut input)
        .map_err(|e| ParseInputError::IoError(e.to_string()))?;

    println!("Saisie : {}", input.trim());

    let per_hour = get_value(&input)?;
    let worked_hours = get_number_of_hours(&input)?;

    Ok((per_hour, worked_hours))
}

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

pub fn get_already_paid_amount() -> Result<f64, ParseInputError> {
    let mut input = String::new();
    println!("\n➡️ Entrez le montant déjà payé.");
    println!("   Exemple : 500");

    io::stdin()
        .read_line(&mut input)
        .map_err(|e| ParseInputError::IoError(e.to_string()))?;

    input
        .trim()
        .replace(',', ".")
        .parse::<f64>()
        .map_err(|_| ParseInputError::InvalidNumber)
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
