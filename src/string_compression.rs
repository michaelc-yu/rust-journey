

fn string_compression(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut compressed = String::new();
    let mut chars = s.chars().peekable();

    // chars.next() returns the next char wrapped in Option
    // unwrap() takes an Option and returns the value if it's Some, or panics if it's None
    // here we take the first char of the string, which is safe because we check above that string isn't
    // empty so unwrap will never panic
    let mut current_char = chars.next().unwrap();
    let mut count = 1;

    while let Some(&next_char) = chars.peek() {
        if next_char == current_char {
            count += 1;
            chars.next(); // consume the character
        } else {
            compressed.push(current_char);
            compressed.push_str(&count.to_string());
            current_char = chars.next().unwrap();
            count = 1;
        }
    }
    // add the last character and its count
    compressed.push(current_char);
    compressed.push_str(&count.to_string());

    compressed
}

fn main() {
    let s = "hello";
    println!("{}", string_compression(&s.to_string()));
    assert_eq!("h1e1l2o1", string_compression(&s.to_string()));

    let s2 = "aaabbc";
    println!("{}", string_compression(&s2.to_string()));
    assert_eq!("a3b2c1", string_compression(&s2.to_string()));

    let s3 = "mmmoomoooo";
    println!("{}", string_compression(&s3.to_string()));
    assert_eq!("m3o2m1o4", string_compression(&s3.to_string()));

    let s4 = "";
    println!("{}", string_compression(&s4.to_string()));
    assert_eq!("", string_compression(&s4.to_string()));
}

