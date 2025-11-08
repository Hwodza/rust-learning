use std::io;
fn main() {
    println!("Temperature Converter");
    loop {
        println!("Do you want to convert (1) from F to C or (2) C to F");
        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");
        let guess: u32 = match unit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type 1 or 2");
                continue;
            },
        };
        let start_unit: char;
        let end_unit: char;
        if guess == 1 {
            start_unit = 'F';
            end_unit = 'C';
        } else if guess == 2 {
            start_unit = 'C';
            end_unit = 'F';
        } else {
            println!("Please type 1 or 2");
            continue;
        }
        println!("Please enter starting temperature in {start_unit}");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        let temp: f64 = match  temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };

        if guess == 1 {
            println!("{temp} {start_unit} is {} {end_unit}", (temp-32.0)*(5.0/9.0));
        } else {
            println!("{temp} {start_unit} is {} {end_unit}", temp*1.8+32.0);
        }
    }
}
