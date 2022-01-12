fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn nothing() -> () {
    return ();
}

fn nothing2() {
    return ();
}

fn main() {
    println!("{}", add(42, 13));
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
    let a = nothing();
    let b = nothing2();
    println!("a の値 {:?}", a);
    println!("b の値 {:?}", b);
}
