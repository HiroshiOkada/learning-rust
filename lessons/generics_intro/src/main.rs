fn print_anything<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}

fn main() {
    print_anything(10);       // 整数
    print_anything(3.14);     // 浮動小数点数
    print_anything("hello");  // 文字列スライス
    print_anything(vec![1, 2, 3]); // ベクタ
}