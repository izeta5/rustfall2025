const FREEZE: f64 = 32.0;

// Converts F -> C
fn fahrenheit_to_celsius(f: f64) -> f64 {
    let celsius: f64 = (f - FREEZE) / 1.8;
    return celsius;
}

// Converts C -> F
fn celsius_to_fahrenheit(c: f64) -> f64 {
    let fahrenheit: f64 = 1.8 * c + FREEZE;
    return fahrenheit;
}

// Checks if num is even
fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

// Checks if guess == secret num
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        return 0;
    } else if guess > secret {
        return 1;
    } else {
        return -1;
    }
}

// ------------------ Problem 1 ------------------
fn problem_1() {
    println!("\nProblem 1. Temperature Converter:\n");

    let mut temp: f64 = 20.0; 
    println!("{}C -> {:.3}F", temp as i32, celsius_to_fahrenheit(temp));

    let mut temp: f64 = 35.0; 
    println!("{}F -> {:.3}C", temp as i32, fahrenheit_to_celsius(temp));

    for i in (temp as i32 + 1)..=(temp as i32 + 5) {
        println!("{}F -> {:.3}C", i, fahrenheit_to_celsius(i as f64));
    }
}

// ------------------ Problem 2 ------------------
fn problem_2() {
    println!("\nProblem 2. Number Analyzer:\n");

    let nums: [i32; 10] = [2, 7, 13, 5, 1, 6, 15, 0, 9, 3];

    for i in 0..nums.len() {
        if is_even(nums[i]) {
            println!("{} is even.", nums[i]);
        } else {
            if (nums[i] % 3 == 0) && (nums[i] % 5 == 0) {
                println!("FizzBuzz ({})", nums[i]);
            } else if nums[i] % 3 == 0 {
                println!("Fizz ({})", nums[i]);
            } else if nums[i] % 5 == 0 {
                println!("Buzz ({})", nums[i]);
            } else {
                println!("{} is odd.", nums[i]);
            }
        }
    }

    let mut sum: i32 = 0;
    let mut big: i32 = nums[0];
    let mut index = 0;

    while index < nums.len() {
        if nums[index] > big {
            big = nums[index];
        }
        sum += nums[index];
        index += 1;
    }

    println!("\nFinal Sum = {}", sum);
    println!("Largest Number = {}", big);
}

// ------------------ Problem 3 ------------------
fn problem_3() {
    println!("\nProblem 3. Guessing Game:\n");

    let mut secret_num: i32 = 33;
    let mut guess_count: i32 = 0;
    let guesses: [i32; 5] = [7, 50, 25, 35, 33]; // simulate user guesses

    for i in 0..guesses.len() {
        guess_count += 1;

        if check_guess(guesses[i], secret_num) == 0 {
            println!("You guessed it! The secret number was: {}", guesses[i]);
            break;
        } else if check_guess(guesses[i], secret_num) == 1 {
            println!("{} was too high!", guesses[i]);
        } else {
            println!("{} was too low!", guesses[i]);
        }
    }

    println!("\nThat took {} guesses.\n", guess_count);
}

// ------------------ Main ------------------
fn main() {
    println!("\nAssignment 1");
    problem_1();
    problem_2();
    problem_3();
}