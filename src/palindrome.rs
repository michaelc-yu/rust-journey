
// check if a given string is a palindrome (ignoring case and spaces)
fn is_palindrome(s: &str) -> bool {
    let s_lower = s.to_lowercase();
    let filtered: Vec<char> = s_lower.chars()
    .filter(|c| !c.is_whitespace())
    .collect();
    filtered.iter().eq(filtered.iter().rev())
}

fn main() {
    let x = "qwerewq";
    let y = "hello";
    let z = "A b ba";
    println!("{} : {}", x, is_palindrome(x));
    println!("{} : {}", y, is_palindrome(y));
    println!("{} : {}", z, is_palindrome(z));
}

