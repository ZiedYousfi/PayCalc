# PayCalc

A Rust calculator for computing salaries with progressive rate increases.

## Description

PayCalc is a command-line utility that calculates payment amounts based on hourly rates and worked hours. The program accounts for rate increases after a certain number of worked hours, making the calculation more accurate for situations where the hourly rate increases with experience or time worked.

## Installation

Make sure you have Rust installed on your system. Then clone the repository and build the project:

```bash
git clone https://github.com/ZiedYousfi/PayCalc.git
cd paycalc
cargo build --release
```

## Usage

Run the program and follow the prompts:

```bash
cargo run
```

The program will ask for:
1. The hourly rate increase amount after each period
2. The number of hours per period before the rate increase
3. The initial hourly rate (preceded by an asterisk `*`) and hours worked
4. The amount already paid (if applicable)

### Input Format

For the main input regarding rate and hours:
- The hourly rate must be preceded by an asterisk (`*`)
- The worked hours should follow the rate, separated by a space

### Usage Example

```
➡️  Entrez le montant de l'augmentation du taux horaire après chaque palier.
   Exemple : 10 (ce qui signifie +10€/h après chaque palier)
5.0

➡️  Entrez le nombre d'heures par palier avant l'augmentation du taux.
   Exemple : 10 (le taux augmente toutes les 10h)
10.0

➡️  Entrez sur une seule ligne le taux horaire (commençant par '*') suivi du nombre d'heures travaillées.
   Exemple : *50 40 (signifie 50€/h pendant 40 heures)
*25 40

➡️ Entrez le montant déjà payé.
   Exemple : 500
0
```

The program will calculate and display:
- A detailed calculation per period
- The total amount earned
- The remaining amount to be paid after deducting the already paid amount

In the example above:
- Initial rate: 25€/h
- Increase period: 10 hours
- Increase per period: 5€/h
- Total hours worked: 40h

The calculation breaks down as follows:
- Period 1: 10h at 25€/h = 250€
- Period 2: 10h at 30€/h = 300€
- Period 3: 10h at 35€/h = 350€
- Period 4: 10h at 40€/h = 400€
- Total: 1300€

## Advanced Features

- Support for decimal numbers with dots or commas
- Detailed calculation by period
- Deduction of amounts already paid
- User error handling

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Zied Yousfi
