fn main() {
    let x = 12;
    let a = 12u8;
    let b = 4.3;
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "Hello World!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);
    let t = true;
    println!("{}", t as u8);
}
