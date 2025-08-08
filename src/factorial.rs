
fn factorial_recursive(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial_recursive(n-1)
    }
}

fn factorial_iterative(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut res = 1;
    for i in 1..(n+1) {
        res *= i
    }
    res
}

fn main() {
    let x = 7;
    let res = factorial_recursive(x);
    println!("factorial of {}: {}", x, res);
    assert_eq!(res, 5040);

    let y = 6;
    let res = factorial_iterative(y);
    println!("factorial of {}: {}", y, res);
    assert_eq!(res, 720);
}
