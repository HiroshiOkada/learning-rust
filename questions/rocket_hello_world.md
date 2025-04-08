# 課題: Rocketで最初のルートを作ろう！🚀

おつかれさま！メガネ君！✨ プロジェクトのセットアップ、バッチリだったね！💯

次は、いよいよRocketを使ってWebページを表示してみよう！
まずは、アクセスしたら「Hello, world!」って表示されるだけの、超シンプルなページを作るよ！

## やることリスト ✅

1.  **`lessons/my_rocket_app/src/main.rs` を開く**:
    このファイルにコードを書いていくよ！

2.  **必要なクレートを `use` する**:
    ファイルの先頭に、Rocketのマクロとかを使うためのおまじないを追加してね！
    ```rust
    #[macro_use] extern crate rocket;
    ```

3.  **ルートハンドラ関数を作る**:
    特定のURL（今回はトップページ `/`）にアクセスがあったときに、何をするかを書く関数だよ。
    `index` っていう名前の関数を作って、「Hello, world!」っていう文字列を返すようにしよう！
    `#[get("/")]` っていうアトリビュートを関数の上に付けるのを忘れずにね！これが「トップページへのGETリクエストに対応するよ」っていう意味になるんだ。
    ```rust
    #[get("/")]
    fn index() -> &'static str {
        "Hello, world!"
    }
    ```

4.  **`main` 関数を修正する**:
    プログラムが起動したときに、Rocketサーバーを立ち上げて、さっき作ったルートハンドラを登録するように `main` 関数を書き換えるよ。
    `#[launch]` っていうアトリビュートを `main` 関数の上に付けると、Rocketがいい感じにサーバーを起動してくれるんだ！超便利！✨
    ```rust
    #[launch]
    fn rocket() -> _ {
        rocket::build().mount("/", routes![index])
    }
    ```
    *   `rocket::build()` でRocketアプリのインスタンスを作るよ。
    *   `.mount("/", routes![index])` で、「トップページ (`/`) にアクセスがあったら、`index` 関数を使ってね」ってRocketに教えてあげるんだ。

5.  **実行して確認！**:
    ターミナルで `lessons/my_rocket_app` ディレクトリに移動して、`cargo run` コマンドを実行してみて！
    コンパイルが終わって、サーバーが起動したら、Webブラウザで `http://127.0.0.1:8000` を開いてみよう！
    「Hello, world!」って表示されたら成功だよ！🎉

## 完成イメージ (`lessons/my_rocket_app/src/main.rs`)

```rust
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
```

分からなかったら、いつでも聞いてね！🙋‍♀️ がんばれー！💪🔥