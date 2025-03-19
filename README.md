# PayCalc

A simple Rust calculator for calculating net salary based on hourly rates and worked hours.

## Description

PayCalc is a command-line utility that calculates payment amounts based on hourly rates and the number of hours worked. It also supports rate increases based on additional hours worked.

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
1. Rate increase amount
2. Hours required for rate increase
3. Basic input text containing hourly rate and worked hours

### Input Format

The input text should follow this format:
- Hourly rate should be preceded by an asterisk (`*`)
- Worked hours should be preceded by whitespace

### Examples

```
Please enter the rate increase:
5.0
Please enter the hours for increase:
10.0
Please enter the per hour wage (starting with "*") and hours worked:
Rate is *25 and hours worked are 40
```

This means:
- Base rate is $25/hour
- After every 10 hours worked, the rate increases by $5
- Total hours worked is 40
- Final rate will be $25 + (40 × $5/10) = $45/hour
- Total payment will be 40 × $45 = $1800

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Zied Yousfi
