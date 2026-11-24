use std::io;

fn farenheit_to_celsius(farenheit_temp: f64) -> f64 {
    (farenheit_temp - 32.0) * 5.0 / 9.0
}

fn celsius_to_farenheit(celsius_temp: f64) -> f64 {
    celsius_temp * 9.0 / 5.0 + 32.0
}

fn print_instructions() {
    println!("1 > Input 1 for Farenheit to Celsius");
    println!("2 > Input 2 for Celsius to Farenheit");
    println!("3 > Input 3 to exit");
}

fn print_validity() {
    println!();
    println!("Invalid CHoice!");
}

fn main() {
    println!("TEMPERATURE CONVERTER");

    //instructions
    print_instructions();

    //the converting loop
    loop {
        //receiving the user's input
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 =  match choice.trim().parse() {
            Ok(1) => 1,
            Ok(2) => 2,
            Ok(3) => break,
            Ok(_) => {
                print_validity();
                print_instructions();
                continue;
            },
            Err(_) => {
                print_validity();
                print_instructions();
                continue;
            }
        };

        //receiving the user's temperature
        if choice == 1 {
            println!("Enter the Farenheit temperature you wish to convert!");
        } else {
            println!("Enter the Celsius temperature you wish to convert!")
        };

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

        //converting the user's temperature
        if choice == 1 {
            let celsius = farenheit_to_celsius(temperature);
            println!("{temperature}°F is {celsius:.2}°C");
            println!();

            //restating the instructions
            print_instructions();
        } else {
            let farenheit = celsius_to_farenheit(temperature);
            println!("{temperature}°C is {farenheit:.2}°F");
            println!();

            //restating the instructions
            print_instructions();
        };
    }
}
