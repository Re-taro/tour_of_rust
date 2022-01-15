struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

struct Location(i32, i32);

struct Marker;

fn main() {
    let s = String::from("Hello World!");
    println!("{} is {} char long.", s, s.len());
    // SeaCreatureのデータはスタック
    let ferris = SeaCreature {
        // String構造体もスタックに入るが
        // ヒープに入るデータの参照アドレスが入る
        animal_type: String::from("crab"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };
    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("none"),
    };
    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
    println!(
        "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs
    );
    // タプルライクな構造体も利用できる
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
    // 空のタプルをユニットと呼ぶ
    // フィールドを持たない構造体はユニットを返す
    let _m = Marker;
}
