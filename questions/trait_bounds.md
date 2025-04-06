# 課題: トレイト境界を使ってジェネリック関数を作ろう！

トレイト境界を使うと、ジェネリック関数が受け取る型に「特定の能力（トレイト）を持っていること」を要求できるよ！✨ これで、関数の中でそのトレイトのメソッドを安全に呼び出せるようになるんだ。

## やってみよう！

1.  **`Describe` トレイトを定義する**:
    *   `describe` という名前のメソッドを持つように定義してね。
    *   `describe` メソッドは、`&self` を引数に取り、`String` 型の値を返すようにする。

2.  **`Book` 構造体を定義する**:
    *   `title` (String) と `author` (String) のフィールドを持つようにしてね。

3.  **`Movie` 構造体を定義する**:
    *   `title` (String) と `director` (String) のフィールドを持つようにしてね。

4.  **`Book` に `Describe` トレイトを実装する**:
    *   `describe` メソッドが `"「[title]」by [author]"` という形式の文字列 (String) を返すように実装する。

5.  **`Movie` に `Describe` トレイトを実装する**:
    *   `describe` メソッドが `"「[title]」directed by [director]"` という形式の文字列 (String) を返すように実装する。

6.  **`print_description` ジェネリック関数を定義する**:
    *   型パラメータ `<T>` を持ち、`T` には `Describe` トレイト境界を付ける。
    *   引数として `item: T` を受け取る。
    *   関数の中では、受け取った `item` の `describe` メソッドを呼び出し、その結果をプリントする。 例: `println!("Description: {}", item.describe());`

7.  **`main` 関数で動作確認**:
    *   `Book` と `Movie` のインスタンスを作って、それぞれ `print_description` 関数に渡して、説明が正しくプリントされるか確認する。

```rust
// ここに Describe トレイトを定義する

// ここに Book 構造体を定義する

// ここに Movie 構造体を定義する

// ここに Book への Describe トレイトの実装を書く

// ここに Movie への Describe トレイトの実装を書く

// ここに print_description ジェネリック関数を定義する (Describe トレイト境界を忘れずに！)

fn main() {
    let my_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
    };

    let my_movie = Movie {
        title: String::from("Rust in Motion"),
        director: String::from("Some Director"), // 適当な監督名でOK！
    };

    print_description(my_book);
    print_description(my_movie);
}
```

できたら、`lessons/trait_bounds/src/main.rs` っていうファイルにコードを書いて、`cargo run` で実行してみてね！
本と映画の説明がちゃんとプリントされるか確認して、結果を教えて！💖