fn main() {
    //引用与解引用
    let x = 5;
    let y = &x;
    println!("{}", *y);

    //引用
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", *r1, *r2);

    let s_ref = dangle();
}

fn dangle() -> &String {
    let s = String::from("s");
    &s
}