
fn collatz_seq_len(mut n: u32) -> u32 {
    let mut len: u32 = 1;
    while n != 1 {
        if n % 2 == 1 {
            n = 3 * n + 1;
        } else {
            n = n / 2;
        }
        len += 1;
    }
    len
}

fn main() {
    let a = collatz_seq_len(6);
    println!("{}", a);
    assert_eq!(a, 9);

    let b = collatz_seq_len(15);
    println!("{}", b);
    assert_eq!(b, 18);

    let c = collatz_seq_len(4);
    println!("{}", c);
    assert_eq!(c, 3);
}

