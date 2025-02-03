use std::io;

fn main() {
    println!("Welcome to the farenheit-celsius converter!");
    println!("-------------------------------------------");
    println!("Farenheit to celsius: 1");
    println!("Celsius to farenheit: 2");
    
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
            println!("Convertion: {converter} (Farenheit to celsius)");
            println!("Enter the farenheit value:");

            let mut farenheit = String::new();

            io::stdin()
                .read_line(&mut farenheit)
                .expect("You should enter a valid farenheit value!");

            let farenheit: f64 = match farenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let celsius = (farenheit - 32.0) / (9.0/5.0);

            println!("Celsius value: {celsius}!");
            break;
        } else if converter == 2 {
            println!("Convertion: '{converter}' (Celsius to farenheit)");
            println!("Enter the celsius value:");

            let mut celsius = String::new();

            io::stdin()
                .read_line(&mut celsius)
                .expect("You should enter a valid celsius value!");

            let celsius: f64 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let farenheit = (celsius * (9.0/5.0)) + 32.0;

            println!("Farenheit value: {farenheit}");
            break;
        }
    }
}
