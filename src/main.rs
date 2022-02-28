fn main() {
    let a = [1, 2, 3, 4, 5];
    if a[2] > 3 {
        println!(">");
    } else if a[2] < 3 {
        println!("<")
    } else {
        println!("=");
    }

    for i in a {
        print!("{} ", i);
    }

    println!();

    let mut i = 0;
    while i < 5 {
        print!("{} ", a[i]);
        i += 1;
    }
    println!();
    // loop 无条件循环
    let mut ii = 0;
    loop {
        print!("{} ", ii);
        ii += 1;
        if ii == 10 {
            break;
        }
    }
}
