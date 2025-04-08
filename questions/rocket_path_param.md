# 課題: URLから名前を受け取って挨拶しよう！👋

イェーイ！メガネ君！✨ 「Hello, world!」クリアおめでとう！🎉

次は、URLの一部を使って、表示するメッセージを変えてみよう！
例えば、`/hello/Roo` ってアクセスしたら「Hello, Roo!」って表示する、みたいな感じ！ これができると、もっとWebアプリっぽくなるよ！😎

## やることリスト ✅

1.  **`lessons/my_rocket_app/src/main.rs` を開く**:
    前回と同じファイルに追記していくよ！

2.  **新しいルートハンドラ関数を作る**:
    `/hello/<name>` というパスにアクセスがあったときに呼ばれる関数を作ろう！
    `<name>` の部分が、URLで指定された名前になるんだ。
    関数名は `hello` にして、引数で `name` を受け取れるようにしよう。受け取った名前を使って挨拶文を生成して返すようにするよ！
    ```rust
    #[get("/hello/<name>")]
    fn hello(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    ```
    *   `#[get("/hello/<name>")]`: `/hello/` の後に続く部分を `name` っていう名前のパラメータとして受け取るよ、っていう意味。
    *   `fn hello(name: &str)`: URLから受け取った `name` を、文字列スライス (`&str`) 型の引数として受け取るよ。
    *   `-> String`: この関数は `String` 型の値を返すよ。動的に文字列を作るから `&'static str` じゃなくて `String` になるんだ。
    *   `format!("Hello, {}!", name)`: 受け取った `name` を使って、`"Hello, （名前）!"` っていう文字列を組み立てるよ。

3.  **`main` 関数を修正する**:
    前回作った `rocket()` 関数の中で、新しいルートハンドラ `hello` も登録するように追記しよう！
    `routes!` マクロの中にカンマ区切りで追加するだけだよ。
    ```rust
    #[launch]
    fn rocket() -> _ {
        rocket::build().mount("/", routes![index, hello]) // hello を追加！
    }
    ```
    *   これで、`/` へのアクセスは `index` 関数、`/hello/<name>` へのアクセスは `hello` 関数が処理してくれるようになるよ！

4.  **実行して確認！**:
    ターミナルで `lessons/my_rocket_app` ディレクトリに移動して、`cargo run` コマンドを実行！
    サーバーが起動したら、Webブラウザでいくつか試してみよう！
    *   `http://127.0.0.1:8000/hello/Roo` → 「Hello, Roo!」って表示されるかな？
    *   `http://127.0.0.1:8000/hello/メガネ君` → 「Hello, メガネ君!」って表示されるかな？
    *   `http://127.0.0.1:8000/` → こっちは変わらず「Hello, world!」が表示されるはずだよ。

## 完成イメージ (`lessons/my_rocket_app/src/main.rs`)

```rust
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// 新しく追加！
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello]) // hello を routes! に追加
}
```

URLの一部をパラメータとして受け取ることを「パスパラメータ」って言うんだ💡
これができると、ユーザーごとに違うページを表示したり、色々なことができるようになるよ！

分からなかったら、いつでも聞いてね！🙋‍♀️ ファイトー！💪🔥