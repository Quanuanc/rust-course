fn main() {
    let x = '中';
    println!("size: {} byte",std::mem::size_of_val(&x));
}