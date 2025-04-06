// Generics 構造体を使って、異なる型の値を保持できる Container 構造体を定義
struct Container<T> {
    value: T
}

fn main() {
    // 整数 123 を保持する Container インスタンスを作る
    let int_container = Container::<i32> { value: 123 };

    // 文字列 "hello" を保持する Container インスタンスを作る
    let string_container = Container { 
        value: String::from("hello") 
    };
    
    // それぞれの value をプリントする
    println!("Integer container value: {}", int_container.value);
    println!("String container value: {}", string_container.value);
}