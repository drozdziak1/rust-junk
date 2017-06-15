use std::io;

/**
 * A soon-to-be temperature converter
 */
pub fn main() {

    let temp: f64;

    println!("Welcome to the mighty temperature converter!");
    println!("So, how hot is it?");
    loop {
        let mut temp_str = String::new();

        io::stdin()
            .read_line(&mut temp_str)
            .expect("Could not read stdin");

        match temp_str.trim().parse::<f64>() {
            Ok(n) => {
                temp = n;
                break;
            }
            Err(_) => println!("Please enter a *number*"),
        };
    }

    println!("{}, got it!", temp);

    let temp_unit: &str;

    loop {
        let mut unit_str = String::new();

        println!("Is that Celsius(C), Fahrenheit(F), or Kelvin(K)?");

        io::stdin()
            .read_line(&mut unit_str)
            .expect("Could not read stdin");

        let unit_trimmed = unit_str.trim();

        if unit_trimmed.chars().count() > 1 {
            println!("One char's enough!");
            continue;
        }

        temp_unit = match unit_trimmed.to_uppercase().chars().next() {
            Some('C') => "Celsius",
            Some('F') => "Fahrenheit",
            Some('K') => "Kelvin",
            None => {
                println!("You entered an empty string!");
                continue;
            }
            default => {
                println!("Can't understand {:?}", default.unwrap());
                continue;
            }
        };
        break;
    }

    println!("About to convert from {} degrees", temp_unit);
}
