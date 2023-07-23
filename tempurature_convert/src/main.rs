use std::io;

fn main() {
    // conversion temperature

    loop {
        let mut temp = String::new();

        println!("\nEnter a Celcius or Farenheit temp and it will convert it to the other temp");
        println!("Example: 48C or 92.6F");

        // read in user input, if its junk, throw and error
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read the temperature");

        // trim whitespaces from temp
        let temp = temp.trim();

        //  check for invalid input (must be at least 2 chars number + unit)
        if temp.len() < 2 {
            println!("Invalid input, please enter a number AND a unit (C/F)");
            continue;
        }

        // split the input into two parts temp value and temp type
        // the C or F will always be at the end so that will be our guide for splitting
        let (temp_num, temp_value) = temp.split_at(temp.len() - 1);

        // trim and force uppercase on the 'value' character
        let temp_value = temp_value.trim().to_uppercase();

        // check to ensure that the number is still a value otherwise ask them again to enter a tempurature
        // if good, pass it to the variable _number
        let _number: f32 = match temp_num.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a numeric value");
                continue;
            }
        };

        // validate unit (C or F only)
        let _unit = match temp_value.as_str() {
            "C" => {
                println!("Converting Celcius to Farenheit");
                'C'
            }
            "F" => {
                println!("Converting Farenheit to Celcius");
                'F'
            }
            
            // if its not C or F, catch the error here
            _ => {
                println!("Please enter either C(elcius) or F(arenheit) for your temperatures");
                continue;
            }
        };
        
        // if its F convert to C, otherwise convert to F
        if temp_value == "F" {
            // print the tempurature in C, format it to 1 decimal place
            println!(
                "{} in Celcius is: {}C",
                temp,
                format!("{:.1}", convert_to_celcius(_number))
            )
        } else {
            println!(
                "{} in Farenheit is: {}F",
                temp,
                format!("{:.1}", convert_to_farenheit(_number))
            )
        }
        break;
    }
}

fn convert_to_celcius(_number: f32) -> f32 {
    (_number - 32.0) * 0.555556
}
fn convert_to_farenheit(_number: f32) -> f32 {
    _number * 1.8 + 32.0
}
