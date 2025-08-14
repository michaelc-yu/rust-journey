
// Write a function to rotate the elements of a Vec<i32> to the right by k steps.
// Example:
// [1, 2, 3, 4, 5], k = 2 → [4, 5, 1, 2, 3]

fn rotate_vec(v: Vec<i32>, k: usize) -> Vec<i32> {
    let len = v.len();
    if len == 0 { return v; }
    let k = k % len;
    let mut res = Vec::with_capacity(len);
    res.extend_from_slice(&v[len - k..]);
    res.extend_from_slice(&v[..len - k]);
    res
}

fn main() {
    let res = rotate_vec(vec![1, 2, 3, 4, 5], 2);
    println!("{:?}", res);
    assert_eq!(res, vec![4, 5, 1, 2, 3]);

    let res = rotate_vec(vec![1, 2, 3, 4, 5], 5);
    println!("{:?}", res);
    assert_eq!(res, vec![1, 2, 3, 4, 5]);
}
