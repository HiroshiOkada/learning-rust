# 課題: ジェネリック構造体を作ってみよう！

ジェネリクスは構造体にも使えるよ！✨
いろんな型の値を保持できる、柔軟なデータ構造を作ってみよう！

## やってみよう！

`Container` という名前のジェネリック構造体を定義してみて！
この構造体は、型パラメータ `<T>` を持ち、型 `T` の値を一つだけ保持する `value` というフィールドを持つようにしてね。

`main` 関数の中では、
1.  `Container` を使って、整数 `123` を保持するインスタンス `int_container` を作る。
2.  `Container` を使って、文字列 `"hello"` (String型) を保持するインスタンス `string_container` を作る。
3.  それぞれのコンテナの `value` フィールドをプリントして、中身を確認する。

```rust
// ここにジェネリック構造体 Container を定義する！
// 型パラメータ T を使ってね！

fn main() {
    // 整数 123 を保持する Container インスタンスを作る
    let int_container = // ...

    // 文字列 "hello" を保持する Container インスタンスを作る
    let string_container = // ... String::from("hello") を使おう！

    // それぞれの value をプリントする
    println!("Integer container value: {}", int_container.value);
    println!("String container value: {}", string_container.value);
}
```

できたら、`lessons/generics_struct/src/main.rs` っていうファイルにコードを書いて、`cargo run` で実行してみてね！
整数と文字列がちゃんとプリントされるか確認して、結果を教えて！💖