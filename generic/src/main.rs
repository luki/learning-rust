fn someWhat<T, U>(type1: T, type2: U) -> (U, T) {
    (type2, type1)
}

fn main() {
    let haha = someWhat("Hello", 5);
    println!("{}", haha.1);
}
