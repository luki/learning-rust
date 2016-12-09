struct Character {
    age: i32,
    height: i32
}

fn main() {
    let human = Character {
        age: 16,
        height: 179
    };

    println!("{}", human.age);
}
