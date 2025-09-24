fn concat_strings(s1: &String, s2: &String) -> String {
    let mut new_str: String = s1.to_string();
    new_str.push_str(&s2);
    return new_str
}

fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone();
    cloned.push_str("World!");
    return cloned
}

fn sum(total: &mut i32, low: i32, high: i32) {
    let mut acc = 0;
    for i in low..=high {
        acc += i;
    }
    *total = acc;
}

fn problem1() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}

fn problem2() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}

fn problem3() {
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Sum from 0 to 100: {}", total); // Should print: 5050
}

fn main() {
    problem1();
    problem2();
    problem3();
}
