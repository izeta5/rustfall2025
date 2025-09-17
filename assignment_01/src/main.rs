const FREEZE: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    let celsius: f64 = (f - FREEZE) / 1.8;
    return celsius
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    let farhenheit: f64 = 1.8 * c + FREEZE;
    return farhenheit
}

fn is_even(n: i32) -> bool{
    if n % 2 == 0{
        return true
    }
    else{
        return false
    }
}

fn main() {
    
    println!("Assignment 1");
    println!("Problem 1. Temperature Converter:\n");
    let mut temp: f64 = 35.0;
    println!("{}F -> {:.3}C\n",temp as i32,fahrenheit_to_celsius(temp));
    for i in (temp as i32 + 1)..(temp as i32 + 6){
        println!("{}F -> {:.3}C",i,fahrenheit_to_celsius(i as f64))
    }
    
    println!("\nProblem 2. Number Analyzer:\n");
    let nums: [i32;10] = [2, 7, 13, 5, 1, 6, 15, 10, 9, 3];
    for i in 0..nums.len(){
        println!("{}",is_even(nums[i]));
    }
}

//PUT EACH ASSIGNMENT IN A FUNCTION. HAVE MAIN ONLY CALLING 3 FUNCTIONS THATS IT!