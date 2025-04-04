use std::num::ParseIntError;

fn string_to_int(s: &str) -> Result<i32, ParseIntError> {
    let num = s.parse::<i32>();
    num
}

fn main() {
    match string_to_int("10") {
        Ok(n) => {
            println!("num: {}", n);
        }
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };


    match string_to_int("hello") {
        Ok(n) => {
            println!("num: {}", n);
        }
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}