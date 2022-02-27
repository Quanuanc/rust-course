fn main() {
    let a = 10;
    let b: i32 = 20;
    let mut c = 30i32; //可变变量
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("(a+b)+(c+d)={}", e);
}

fn add(i: i32, j: i32) -> i32 {
    i + j //没有分号，返回
}
