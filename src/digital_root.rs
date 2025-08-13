

fn digital_root(n: u64) -> u64 {
    let mut root: u64 = n;
    while root.to_string().len() > 1 {
        let mut sum: u64 = 0;
        let s = root.to_string();
        for c in s.chars() {
            if let Some(digit) = c.to_digit(10) {
                sum += digit as u64;
            }
        }
        root = sum;
    }
    root
}


fn main() {
    println!("{}", digital_root(12345));
    assert_eq!(6, digital_root(12345));

    println!("{}", digital_root(942));
    assert_eq!(6, digital_root(942));

    println!("{}", digital_root(98));
    assert_eq!(8, digital_root(98));

    println!("{}", digital_root(300));
    assert_eq!(3, digital_root(300));
}
