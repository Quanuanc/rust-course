fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); //这里会报错

    let x: &str = "hello world";
    let y = x;
    println!("{},{}", x, y);

    let s1 = String::from("nihao");
    let s2 = s1.clone();
    println!("{},{}", s1, s2);

    let s = String::from("hello");
    take_ownership(s);
    // println!("{}", s); //这里会报错

    let i = 15;
    makes_copy(i);
    println!("{}", i);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}