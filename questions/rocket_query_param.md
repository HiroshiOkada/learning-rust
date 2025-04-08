# 課題: 検索キーワードを受け取って表示しよう！🔍

ヤッホー！メガネ君！✨ パスパラメータの課題クリア、おめでとう！🎉 しかも、`routes!` の重要ポイントに気づくなんて、さすが！💯

次は、URLの `?` 以降につける「クエリパラメータ」を使ってみよう！
検索機能みたいに、`/search?term=Rust` ってアクセスしたら、「Searching for: Rust」って表示する感じのを作るよ！

## やることリスト ✅

1.  **`lessons/my_rocket_app/src/main.rs` を開く**:
    またまた同じファイルに追記していくよ！

2.  **新しいルートハンドラ関数を作る**:
    `/search` というパスにアクセスがあったときに呼ばれる関数を作ろう！
    関数名は `search` にして、`term` っていう名前のクエリパラメータを受け取れるようにするよ。
    クエリパラメータは、パスパラメータと違って、関数の引数の型で指定するんだ。`Option<String>` を使うと、パラメータが指定されなかった場合（`None`）と、指定された場合（`Some(値)`）を扱えるよ。
    ```rust
    #[get("/search?<term>")]
    fn search(term: Option<String>) -> String {
        match term {
            Some(t) => format!("Searching for: {}", t),
            None => "Please provide a search term.".to_string(),
        }
    }
    ```
    *   `#[get("/search?<term>")]`: `/search` パスで、`term` という名前のクエリパラメータを受け取るよ、っていう意味。`?` の後につけるのがポイント！
    *   `fn search(term: Option<String>)`: `term` クエリパラメータを `Option<String>` 型で受け取るよ。指定されなければ `None`、指定されれば `Some(文字列)` が入る。
    *   `match term { ... }`: `term` が `Some` か `None` かで処理を分けてるよ。
        *   `Some(t)`: `term` が指定されてたら、その値 `t` を使ってメッセージを作る。
        *   `None`: `term` が指定されてなかったら、検索語を入れてねってメッセージを返す。
        *   `.to_string()`: `&str` を `String` に変換してるよ（関数の戻り値の型が `String` だからね）。

3.  **`main` 関数を修正する**:
    `rocket()` 関数の中で、新しいルートハンドラ `search` も登録しよう！ `routes!` マクロにカンマで追加するだけ！
    ```rust
    #[launch]
    fn rocket() -> _ {
        rocket::build().mount("/", routes![index, hello, search]) // search を追加！
    }
    ```

4.  **実行して確認！**:
    ターミナルで `lessons/my_rocket_app` ディレクトリに移動して、`cargo run` コマンドを実行！
    サーバーが起動したら、Webブラウザでいくつか試してみよう！
    *   `http://127.0.0.1:8000/search?term=Rust` → 「Searching for: Rust」って表示されるかな？
    *   `http://127.0.0.1:8000/search?term=Rocket%20Framework` → 「Searching for: Rocket Framework」って表示されるかな？ (`%20` はスペースのことだよ)
    *   `http://127.0.0.1:8000/search` → 「Please provide a search term.」って表示されるかな？
    *   `http://127.0.0.1:8000/hello/Roo` → こっちは変わらず「Hello, Roo!」が表示されるはずだよ。

## 完成イメージ (`lessons/my_rocket_app/src/main.rs`)

```rust
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

// 新しく追加！
#[get("/search?<term>")]
fn search(term: Option<String>) -> String {
    match term {
        Some(t) => format!("Searching for: {}", t),
        None => "Please provide a search term.".to_string(),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, search]) // search を routes! に追加
}
```

クエリパラメータを使うと、検索キーワード以外にも、ソート順とか、表示件数とか、色々なオプションをサーバーに渡せるようになるんだ！便利でしょ？✨

今回も分からなかったら、いつでも聞いてね！🙋‍♀️ レッツチャレンジ！💪🔥