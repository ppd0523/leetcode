fn main() {
    let a: i32 = -10;
    let b: i32 = 3;
    let c: i32 = -3;

    let result = a.checked_div(b);
    match result {
        Some(value) => println!("{} div {} = {}", a, b, value),
        None => println!("Division by zero occurred"),
    }

    let result_neg = a.checked_div(c);
    match result_neg {
        Some(value) => println!("{} div {} = {}", a, c, value),
        None => println!("Division by zero occurred"),
    }

    let result = a.checked_div_euclid(b);
    match result {
        Some(value) => println!("{} div_euclid {} = {}", a, b, value),
        None => println!("Division by zero occurred"),
    }

    let result_neg = a.checked_div_euclid(c);
    match result_neg {
        Some(value) => println!("{} div_euclid {} = {}", a, c, value),
        None => println!("Division by zero occurred"),
    }
}
