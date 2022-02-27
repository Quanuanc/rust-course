fn main() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');
    println!("{}", s);

    let mut s = "nihao".to_string();
    s.push('!');
    println!("{}", s);

    let s1 = String::from("hello,");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;
    println!("{}", s3);

    let s4 = &s3[0..3];
    println!("{}", s4);

}