use std::io::{self, Write};

fn main() {
    println!("===== Temperature conversion program =====");
    loop{
        println!("\nChoose the type of conversion: 
        1. Celsius -> Fahrenheit
        2. Celsius -> Kelvin
        3. Fahrenheit -> Celsius
        4. Fahrenheit -> Kelvin
        5. Kelvin -> Celsius
        6. Kelvin -> Fahrenheit
        To exit enter 0");

        let degree_symbol = "\u{B0}"; 
        let mut choice = String::new(); 

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim(); 
        
        if choice == "1" {

            println!("C -> F");
            print!("Enter temp in Celsius: ");
            io::stdout().flush().expect("Failed to flush");

            let mut temp_in_c = String::new(); 

            io::stdin()
                .read_line(&mut temp_in_c)
                .expect("Failed to read line");

            let temp_in_c: f64 = match temp_in_c.trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid temperature. Please enter a number.");
                    continue;
                }
            };

            let temp_in_f = (9.0 * temp_in_c) / 5.0 + 32.0; 
            println!("{temp_in_c}{degree_symbol}C = {temp_in_f}{degree_symbol}F");

        } else if choice == "2" {

            println!("C -> K");
            print!("Enter temp in Celsius: ");
            io::stdout().flush().expect("Failed to flush"); 

            let mut temp_in_c = String::new(); 

            io::stdin()
                .read_line(&mut temp_in_c)
                .expect("Failed to read line"); 

            let temp_in_c: f64 = match temp_in_c.trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid temperature. Please enter a number.");
                    continue;
                }
            };

            let temp_in_k = temp_in_c + 273.15; 
            println!("{temp_in_c}{degree_symbol}C = {temp_in_k}K");

        } else if choice == "3" {

            println!("F -> C");
            print!("Enter temp in Fahrenheit: ");
            io::stdout().flush().expect("Failed to flush"); 

            let mut temp_in_f = String::new(); 

            io::stdin()
                .read_line(&mut temp_in_f)
                .expect("Failed to read line"); 

            let temp_in_f: f64 = match temp_in_f.trim().parse(){
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid temperature. Please enter a number.");
                    continue;
                }
            };

            let temp_in_c = (temp_in_f - 32.0) * (5.0 / 9.0); 
            println!("{temp_in_f}{degree_symbol}F = {temp_in_c}{degree_symbol}C");

        } else if choice == "4" {

            println!("F -> K");
            print!("Enter temp in Fahrenheit: ");
            io::stdout().flush().expect("Failed to flush"); 

            let mut temp_in_f = String::new(); 

            io::stdin()
                .read_line(&mut temp_in_f)
                .expect("Failed to read line"); 

            let temp_in_f: f64 = match temp_in_f.trim().parse(){
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid temperature. Please enter a number.");
                    continue;
                }
            };

            let temp_in_k = ((temp_in_f - 32.0) * (5.0 / 9.0)) + 273.15; 
            println!("{temp_in_f}{degree_symbol}F = {temp_in_k}K");

        } else if choice == "5" {

            println!("K -> C");
            print!("Enter temp in Kelvin: ");
            io::stdout().flush().expect("Failed to flush");

            let mut temp_in_k = String::new(); 

            io::stdin()
                .read_line(&mut temp_in_k)
                .expect("Failed to read line");

            let temp_in_k: f64 = match temp_in_k.trim().parse(){
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid temperature. Please enter a number.");
                    continue;
                }
            };

            let temp_in_c = temp_in_k - 273.15; 
            println!("{temp_in_k}K = {temp_in_c}{degree_symbol}C");

        } else if choice == "6" {

            println!("K -> F");
            print!("Enter temp in Kelvin: ");
            io::stdout().flush().expect("Failed to flush");

            let mut temp_in_k = String::new(); 

            io::stdin()
                .read_line(&mut temp_in_k)
                .expect("Failed to read line");

            let temp_in_k: f64 = match temp_in_k.trim().parse(){
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid temperature. Please enter a number.");
                    continue;
                }
            };

            let temp_in_f = (temp_in_k - 273.15) * (9.0/5.0) + 32.0; 
            println!("{temp_in_k}K = {temp_in_f}{degree_symbol}F");

        } else if choice == "0"{

            break; 

        } else {

            println!("Invalid input!!!");

        }
    }
}