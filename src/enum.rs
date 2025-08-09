
enum Operation { Add, Subtract, Multiply, Divide }

fn calculate(op: Operation, x: f64, y: f64) -> f64 {
    match op {
        Operation::Add => x + y,
        Operation::Subtract => x - y,
        Operation::Multiply => x * y,
        Operation::Divide => x / y
    }
}

fn main() {
    let a = 10.0;
    let b = 5.0;

    println!("Add: {}", calculate(Operation::Add, a, b));
    println!("Subtract: {}", calculate(Operation::Subtract, a, b));
    println!("Multiply: {}", calculate(Operation::Multiply, a, b));
    println!("Divide: {}", calculate(Operation::Divide, a, b));
}
