

fn fibonacci(n: usize) -> Vec<u64> {
    if n == 1 {
        return vec![0];
    }
    if n == 2 {
        return vec![0, 1];
    }
    let mut v = vec![0, 1];
    for i in 2..(n+1) {
        let term = v[i-2] + v[i-1];
        v.push(term);
    }
    v
}

fn main() {
    let fibonacci_5: Vec<u64> = fibonacci(5);
    println!("{:?}", fibonacci_5);
    assert_eq!(fibonacci_5, vec![0, 1, 1, 2, 3, 5]);

    let fibonacci_7: Vec<u64> = fibonacci(7);
    println!("{:?}", fibonacci_7);
    assert_eq!(fibonacci_7, vec![0, 1, 1, 2, 3, 5, 8, 13]);
}
