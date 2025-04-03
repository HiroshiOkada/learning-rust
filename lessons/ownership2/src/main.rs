fn main() {
    let x = 5; // xはi32型の値5の所有権を持つ
    let y = &x; // yはxのxの参照を借用する

    assert_eq!(5, x); 
    assert_eq!(5, *y); // yは参照なので*を使って値を取得する

    let mut s = String::from("hello"); // sはString型の値の所有権を持つ、Stringは今は("hello")の値を持つ

    change(&mut s);

    println!("{}", s);

    let mut s = String::from("hello"); // 新しいsを作成する

    {
        // r1とr2はsの不変な参照を借用する。不変な参照は複数作成できる
        let r1 = &s;
        let r2 = &s;

        println!("{} and {}", r1, r2);

        // let r3 = &mut s; // エラー！可変な参照は不変な参照と同時に作成できない

    }
    
    let r3 = &mut s; // r1とr2のスコープが終了したので、r3はsの可変な参照を借用することができる
    println!("{}", r3);
}

fn change(some_string: &mut String) { // some_stringはString型の可変な参照を借用する
    some_string.push_str(", world");
}