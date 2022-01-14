struct _SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let s = String::from("Hello World!");
    println!("{} is {} char long.", s, s.len());
}
