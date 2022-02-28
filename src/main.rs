fn main() {
    let f = Some(5);
    let s = plus_one(f);
    let n = plus_one(None);
    println!("{:?}, {:?}, {:?}", f, s, n);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
