use std::io;

// Type d'erreur personnalisé
#[derive(Debug)]
enum ParseInputError {
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

fn main() {
    println!("Hello, world!");

    let increase_for_rate = match get_rate_increase() {
        Ok(rate) => rate,
        Err(e) => {
            eprintln!("Erreur lors de la saisie de l'augmentation de taux : {e}");
            0.0
        }
    };

    let increase_rate_in_hours = match get_hours_for_increase() {
        Ok(hours) => hours,
        Err(e) => {
            eprintln!("Erreur lors de la saisie du nombre d'heures par palier : {e}");
            0.0
        }
    };

    println!(
        "\nTaux d'augmentation : +{increase_for_rate} après chaque {increase_rate_in_hours} heures."
    );

    let (per_hour, worked_hours) = match get_wage_and_hours() {
        Ok(result) => result,
        Err(e) => {
            eprintln!(
                "Erreur lors de la saisie du salaire horaire et des heures travaillées : {e}"
            );
            (0.0, 0.0)
        }
    };

    println!("\nTaux de départ : {per_hour}€/h");
    println!("Nombre total d'heures travaillées : {worked_hours}");

    let to_pay = calculate_payment(
        per_hour,
        worked_hours,
        increase_for_rate,
        increase_rate_in_hours,
    );

    let already_paid = match get_already_paid_amount() {
        Ok(amount) => amount,
        Err(e) => {
            eprintln!("Erreur lors de la saisie du montant déjà payé : {e}");
            0.0
        }
    };

    let final_amount = to_pay - already_paid;

    println!("\n🌸 Résumé 🌸");
    println!("Total gagné : {to_pay:.2}€");
    println!("Déjà payé : {already_paid:.2}€");
    println!("Reste à payer : {final_amount:.2}€");
    println!("Merci pour votre travail 💖");
}

fn get_rate_increase() -> Result<f64, ParseInputError> {
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

fn get_hours_for_increase() -> Result<f32, ParseInputError> {
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

fn get_wage_and_hours() -> Result<(f64, f32), ParseInputError> {
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

fn calculate_payment(
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

fn get_already_paid_amount() -> Result<f64, ParseInputError> {
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

fn get_value(string: &str) -> Result<f64, ParseInputError> {
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

fn get_number_of_hours(string: &str) -> Result<f32, ParseInputError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value() {
        assert_eq!(get_value("*100").unwrap(), 100.0);
        assert_eq!(get_value("salary *50 per hour").unwrap(), 50.0);
        assert_eq!(get_value("rate is *75.5").unwrap(), 75.5);
        assert_eq!(get_value("rate is *75,5").unwrap(), 75.5); // Test comma as decimal separator
        assert!(matches!(
            get_value("no asterisk here"),
            Err(ParseInputError::MissingAsterisk)
        ));
        assert!(matches!(
            get_value("* no number"),
            Err(ParseInputError::InvalidNumber)
        ));
        assert!(matches!(
            get_value(""),
            Err(ParseInputError::MissingAsterisk)
        ));
    }

    #[test]
    fn test_get_number_of_hours() {
        assert_eq!(get_number_of_hours("*100 40").unwrap(), 40.0);
        assert_eq!(get_number_of_hours("hours: 8").unwrap(), 8.0);
        assert_eq!(get_number_of_hours("worked\t35\nhours").unwrap(), 35.0);
        assert_eq!(get_number_of_hours("10 20 30").unwrap(), 60.0);
        assert_eq!(get_number_of_hours("worked 7.5 hours").unwrap(), 7.5); // Test decimal point
        assert_eq!(get_number_of_hours("worked 7,5 hours").unwrap(), 7.5); // Test decimal comma
        assert!(matches!(
            get_number_of_hours("no numbers"),
            Err(ParseInputError::NoHoursFound)
        ));
        assert!(matches!(
            get_number_of_hours(""),
            Err(ParseInputError::NoHoursFound)
        ));
    }

    #[test]
    fn test_calculate_payment() {
        // Test with no increase
        assert_eq!(calculate_payment(50.0, 40.0, 0.0, 10.0), 2000.0);

        // Test with increase
        assert!((calculate_payment(50.0, 25.0, 10.0, 10.0) - 1300.0).abs() < 0.001);

        // Test with fractional hours
        assert!((calculate_payment(50.0, 25.5, 10.0, 10.0) - 1355.0).abs() < 0.001);
    }
}
