use num::complex::Complex;

fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc(f32)");
    println!("   0.1 + 0.2: {}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {}", (abc.2).to_bits());
    println!();

    println!("abc(f64)");
    println!("   0.1 + 0.2: {}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {}", (xyz.2).to_bits());
    println!();

    let x: f32 = -22.0;
    let y = x.sqrt();
    if y.is_nan() {
        println!("is nan");
    }

    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 'a'..'z' {
        print!("{} ", i);
    }
    println!();

    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}