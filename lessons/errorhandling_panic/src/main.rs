fn check_age(age: u32) {
    const ADULT_AGE: u32 = 18;
    if age < ADULT_AGE {
        panic!("未成年です。");
    }
}

fn main() {
    check_age(20); // これは panic しないはず
    println!("20歳はOK！");
    check_age(15); // これは panic するはず
    println!("15歳は...ここには到達しないはず！");
}