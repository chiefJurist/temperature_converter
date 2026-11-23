use std::io;

fn farenheit_to_celsius(farenheit_temp: f64) -> f64 {
    (farenheit_temp - 32.0) * 5.0 / 9.0
}

fn celsius_to_farenheit(celsius_temp: f64) -> f64 {
    celsius_temp * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("Temperature Converter");

    println!("Type 1 for Farenheit to Celsius and 2 for Celsius to Farenheit");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 =  match choice.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Please type 1 or 2");
            return;
        }
    };

    println!("Enter the temperature!");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let temperature: f64 = match input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    if choice == 1 {
        let celsius = farenheit_to_celsius(temperature);
        println!("{temperature}°F is {celsius:.2}°C");
    } else if choice == 2 {
        let farenheit = celsius_to_farenheit(temperature);
        println!("{temperature}°C is {farenheit:.2}°F");
    } else {
        println!("Please choose 1 or 2")
    }
}
