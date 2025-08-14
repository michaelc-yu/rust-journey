

fn safe_divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(format!("Cannot divide by 0"))
    } else {
        Ok(x / y)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(format!("Cannot take square root of a negative number: {}", x))
    } else {
        Ok(x.sqrt())
    }
}


fn main() {
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match safe_divide(5.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match safe_sqrt(4.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match safe_sqrt(-9.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

