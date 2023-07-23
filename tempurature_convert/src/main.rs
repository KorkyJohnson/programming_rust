use std::io;

fn main() {
    // conversion temperature
    let mut temp = String::new();
    println!("Enter a Celcius or Farenheit temp and it will convert it to the other temp");
    println!("Example: 48C or 92.6F"); 
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read the temperature");

    // trim whitespaces from temp
    let temp = temp.trim();

    // split the input into two parts temp value and temp type
    // the C or F will always be at the end so that will be our guide for splitting
    let (temp_num, temp_value) = temp.split_at(temp.len() -1);
    // trim and force uppercase on the 'value' character
    let temp_value = temp_value.trim().to_uppercase();
    // check to ensure that the number is still a value otherwise ask them again to enter a tempurature
    // if good, pass it to the variable _number
    let _number: f32 = match temp_num.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a numeric value");
            return;
        }
    };
    let _unit = match temp_value.as_str() {
        "C" => {
            println!("Converting Celcius to Farenheit");
            'C'
        }
        "F" => {
            println!("Converting Farenheit to Celcius");
            'F'
        }
        _ => {
            println!("Please enter either C(elcius) or F(arenheit) for your temperatures");
            return;
        }
    }; 
    println!("temp_num: {} temp_value: {}", temp_num, temp_value);

    if temp_value == "F" {
        println!("{} in Celcius is: {}", temp, format!("{:.1}",convert_to_celcius(_number)))
    // } else {
    //     println!("{} in Farenheit is: {}", temp, convert_to_farenheit)
    }
}

fn convert_to_celcius(temp_num: f32) -> f32{
    (temp_num - 32.0) * 0.555556 
} 