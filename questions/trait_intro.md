# 課題: トレイトを定義して実装してみよう！

トレイトを使うと、いろんな型に共通の振る舞いを定義できるよ！✨
今回は「話す」能力を持つトレイトを作ってみよう！

## やってみよう！

1.  **`Speak` トレイトを定義する**:
    *   `say_hello` という名前のメソッドを持つように定義してね。
    *   `say_hello` メソッドは、`&self` を引数に取り、`String` 型の値を返すようにする。

2.  **`Dog` 構造体を定義する**:
    *   中身は空でOK！ `struct Dog;` みたいにね。

3.  **`Cat` 構造体を定義する**:
    *   こっちも中身は空でOK！ `struct Cat;`

4.  **`Dog` に `Speak` トレイトを実装する**:
    *   `say_hello` メソッドが `"Woof!"` という文字列 (String) を返すように実装してね。

5.  **`Cat` に `Speak` トレイトを実装する**:
    *   `say_hello` メソッドが `"Meow!"` という文字列 (String) を返すように実装してね。

6.  **`main` 関数で動作確認**:
    *   `Dog` と `Cat` のインスタンスを作って、それぞれの `say_hello` メソッドを呼び出し、返ってきた文字列をプリントして確認する。

```rust
// ここに Speak トレイトを定義する

// ここに Dog 構造体を定義する

// ここに Cat 構造体を定義する

// ここに Dog への Speak トレイトの実装を書く

// ここに Cat への Speak トレイトの実装を書く

fn main() {
    let dog = Dog;
    let cat = Cat;

    println!("Dog says: {}", dog.say_hello());
    println!("Cat says: {}", cat.say_hello());
}
```

できたら、`lessons/trait_intro/src/main.rs` っていうファイルにコードを書いて、`cargo run` で実行してみてね！
犬と猫がちゃんと挨拶するか確認して、結果を教えて！💖