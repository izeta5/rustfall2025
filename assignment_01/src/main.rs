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
    if (n % 2 == 0) || (n == 0){
        return true
    }
    else{
        return false
    }
}

fn check_guess(guess:i32, secret: i32){
    
}

fn main() {
    
    println!("Assignment 1");
    println!("\nProblem 1. Temperature Converter:\n");
    let mut temp: f64 = 35.0;
    println!("{}F -> {:.3}C\n",temp as i32,fahrenheit_to_celsius(temp));
    for i in (temp as i32 + 1)..(temp as i32 + 6){
        println!("{}F -> {:.3}C",i,fahrenheit_to_celsius(i as f64))
    }
    
    println!("\nProblem 2. Number Analyzer:\n");
    let nums: [i32;10] = [2, 7, 13, 5, 1, 6, 15, 0, 9, 3];
    for i in 0..nums.len(){
        if is_even(nums[i]) == true{
            println!("{} is even.",nums[i])
        }
        if is_even(nums[i]) == false{
           if (nums[i] % 3 == 0) && (nums[i] % 5 == 0){
            println!("FizzBuzz ({})", nums[i])
            }
            else if (nums[i] % 3 == 0){
                println!("Fizz ({})", nums[i])
            }
            else if (nums[i] % 5 == 0){
                println!("Buzz ({})", nums[i])
            }
            else{
                println!("{} is odd.",nums[i])
            } 
        }
    }
    let mut sum:i32 = 0;
    let mut big:i32 = nums[0];
    let mut index = 0;
    while index < nums.len(){
        if nums[index] > big{
            big = nums[index];
        }
        sum += nums[index];
        index += 1;
    }
    println!("\nFinal Sum = {}",sum);
    println!("\nLargest Number = {}",big);

    println!("\nProblem 3. Guessing Game:\n");
    let mut secret_num:i32 = 33;

}

//PUT EACH ASSIGNMENT IN A FUNCTION. HAVE MAIN ONLY CALLING 3 FUNCTIONS THATS IT!