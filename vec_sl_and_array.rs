fn main() {
    let names = ["Holly", "Lukas", "Ivy"];
    for name in &names {
        println!("{}", name);
    }

    // OR (to learn match)

    let indecies = vec![0, 1, 2, 3 /* Third one doesn't exist */];
    for index in indecies {
        match names.get(index) {
            Some(x) => println!("at {} is {}", index, x),
            None => println!("Crash at {}", index)
        }
    }

}
