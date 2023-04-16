use std::io;

fn main() {
    
    println!("Type the temperature");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Unable to read temperature");
        
    change_temperature(&temperature);

}

fn change_temperature(temperature: &str) -> f64 {
    
    let bytes = temperature.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'C' {
            println!("Your temperature is in Celsius");

            let num = &temperature[..i];
            let num: f64 = match num.trim().parse() {
                Ok(x) => x,
                Err(_) => -1.0
            };
            
            let ans = (9.0/5.0)*num + 32.0;
            println!("Temperture: {ans:.5}F");
            return ans
        }
        if item == b'F' {
            println!("Your temperature is in Farehnheit");

            let num = &temperature[..i];
            let num: f64 = match num.trim().parse() {
                Ok(x) => x,
                Err(_) => -1.0
            };
            let ans = (num - 32.0)*(5.0/9.0);
            println!("Temperature: {ans:.5}C");
            return ans
        }
    
    }
    println!("You typed invalid temperature");

    -1.0
} 
