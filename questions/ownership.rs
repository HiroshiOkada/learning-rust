fn main() {
    let s = String::from("hello");

    takes_ownership(s.clone());

    println!("{}", s);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}