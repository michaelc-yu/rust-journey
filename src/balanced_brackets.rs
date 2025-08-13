

fn balanced_brackets(s: &str) -> bool {
    let mut v = vec![];
    for c in s.chars() {
        if matches!(c, '{' | '[' | '(') {
            v.push(c);
        } else if matches!(c, '}' | ']' | ')') {
            if let Some(prev) = v.pop() {
                if !((prev == '{' && c == '}')
                || (prev == '[' && c == ']')
                || (prev == '(' && c == ')')) {
                    return false
                }
            }
            else {
                return false
            }
        }
    }
    v.is_empty()
}

fn main() {
    println!("{}", balanced_brackets("{[()]}"));
    println!("{}", balanced_brackets("{()]"));
    println!("{}", balanced_brackets("{{[]}}}"));
    println!("{}", balanced_brackets("{[()()[()]]}"));
}
