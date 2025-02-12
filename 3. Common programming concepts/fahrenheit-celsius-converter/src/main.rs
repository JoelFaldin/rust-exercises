use std::io;

fn main() {
    println!("Welcome to the fahrenheit-celsius converter!");
    println!("-------------------------------------------");
    println!("Fahrenheit to celsius: 1");
    println!("Celsius to fahrenheit: 2");
    
    let mut converter = String::new();
    loop {
        println!("Input which conversion you want to do: ");

        io::stdin()
            .read_line(&mut converter)
            .expect("You should input a valid method!");
    
        let converter: u32 = match converter.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if converter == 1 {
            println!("Convertion: {converter} (Fahrenheit to celsius)");
            println!("Enter the fahrenheit value:");
        } else if converter == 2 {
            println!("Convertion: '{converter}' (Celsius to fahrenheit)");
            println!("Enter the celsius value:");
        }

        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("You should enter a valid value!");

        let value: f64 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if converter == 1 {
            // Fahrenheit to celsius:
            let calculated_value = (value - 32.0) / (9.0/5.0);

            println!("Celsius value: {calculated_value}");
        } else if converter == 2 {
            // Celsius to fahrenheit:
            let calculated_value = (value * (9.0/5.0)) + 32.0;

            println!("Fahrenheit value: {calculated_value}");
        }

        break;
    }
}
