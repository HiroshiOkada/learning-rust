fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2);

    //let r3 = &mut s; // エラー！

    //println!("{}", r3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}