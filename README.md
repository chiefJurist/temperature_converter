# Temperature Converter

A simple command-line temperature converter written in Rust, built as a practice project of *The Rust Programming Language* book. It converts temperatures between Fahrenheit and Celsius and keeps running until you choose to exit.

## How it works

When the program starts, it shows a small menu: type 1 to convert Fahrenheit to Celsius, 2 to convert Celsius to Fahrenheit, or 3 to exit. After you pick a conversion, enter the temperature and the program prints the result rounded to two decimal places, then shows the menu again. Invalid menu choices simply bring the menu back, so only choosing 3 ends the program.

## What it demonstrates

This project practices variables and shadowing, `f64` and `u32` types, functions with parameters and return values, `loop` with `break` and `continue`, `if`/`else` expressions, `match` on a `Result` from `parse`, reading user input with `std::io`, and formatting numbers with `{value:.2}`.

## Requirements and how to run

You need Rust and Cargo installed (get them from https://rustup.rs). Clone the repository, move into the project folder, and run `cargo run`. Follow the on-screen menu to convert temperatures.

## Example

If you choose option 1 and enter 98.6, the program prints `98.6°F is 37.00°C`, and if you choose option 2 and enter 100, it prints `100°C is 212.00°F`.