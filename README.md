# PayCalc

A simple Rust calculator for calculating net salary based on hourly rates and worked hours.

## Description

PayCalc is a command-line utility that calculates payment amounts based on hourly rates and the number of hours worked. It also supports rate increases based on additional hours worked.

## Installation

Make sure you have Rust installed on your system. Then clone the repository and build the project:

```bash
git clone https://github.com/yourusername/paycalc.git
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

Example: `Rate is *25 and hours worked are 40`

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Zied Yousfi
