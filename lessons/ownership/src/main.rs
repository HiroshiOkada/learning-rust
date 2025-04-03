fn main() {
    let mut s = String::from("hello");

    s = takes_ownership(s);

    println!("{}", s);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);

    some_string
}