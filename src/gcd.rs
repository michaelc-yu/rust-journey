

fn gcd(a: u64, b: u64) -> u64 {
    let mut x: u64 = a;
    let mut y: u64 = b;
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn main() {
    println!("{}", gcd(12, 24));
    assert_eq!(gcd(12, 24), 12);

    println!("{}", gcd(35, 192));
    assert_eq!(gcd(35, 192), 1);

    println!("{}", gcd(68, 102));
    assert_eq!(gcd(68, 102), 34);
}


