const FREEZE: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    let celsius: f64 = (f - FREEZE) / 1.8;
    return celsius
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    let farhenheit: f64 = 1.8 * c + FREEZE;
    return farhenheit
}

fn main() {
    println!("Assignment 1");
    println!("Problem 1: Temperature Converter:");
    let mut temp: f64 = 35.0;
    println!("{:.4} C",fahrenheit_to_celsius(temp));
    for i in (temp as i32 + 1)..(temp as i32 + 6){
        println!("{:.4} C",fahrenheit_to_celsius(i as f64))
    }


}
