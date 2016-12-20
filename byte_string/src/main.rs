use std::str;

fn main() {
    let s = "Holly";
    let s_bv = s.as_bytes();
    println!("{:?}", s_bv);
    let new_s = match str::from_utf8(s_bv) {
        Ok(x) => x,
        Err(y) => panic!("Error w/ value {}", y)
    };
    println!("{}", new_s);
}
