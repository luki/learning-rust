struct Human {
    age: i32,
    height: i32,
    school_year: i32
}

fn main() {
    let holly = Human {
        age: 14,
        height: 165,
        school_year: 9
    };

    let array = [
        Human {
            age: 14,
            height: 165,
            school_year: 9
        },
        Human {
            age: 16,
            height: 179,
            school_year: 10
        }
    ];

    println!("Holly is {} years old, {} cm tall and currently in grade {}", holly.age, holly.height, holly.school_year);

    for human in &array {
        println!("{}", human.age);
    }

}
