

fn remove_duplicates(v: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    for &elt in v.iter() {
        if !res.contains(&elt) {
            res.push(elt);
        }
    }
    res
}

fn main() {
    let v = vec![1, 2, 2, 3];
    let res = remove_duplicates(v);
    println!("{:?}", res);
    assert_eq!(res, vec![1, 2, 3]);

    let v = vec![4, 6, 1, 4, 1, 7];
    let res = remove_duplicates(v);
    println!("{:?}", res);
    assert_eq!(res, vec![4, 6, 1, 7]);
}


