# 課題: JSONデータを返すAPIを作ろう！📦

イェーイ！メガネ君！✨ クエリパラメータもマスターして、絶好調だね！🎉

次は、Web APIの定番、JSON形式でデータを返すエンドポイントを作ってみよう！
例えば、`/api/user` ってアクセスしたら、ユーザー情報がJSONで返ってくる、みたいな感じのを作るよ！

これには、RustのデータをJSONに変換するライブラリ `serde` と、RocketでJSONを簡単に扱うための機能を使うよ！

## やることリスト ✅

1.  **`Cargo.toml` を編集する**:
    JSONを扱うために、`serde` と `rocket` の `json` フィーチャーが必要だよ。
    `lessons/my_rocket_app/Cargo.toml` を開いて、`dependencies` セクションに以下を追加・修正してね。

    ```toml
    [dependencies]
    rocket = { version = "0.5.0", features = ["json"] } # "json" フィーチャーを追加
    serde = { version = "1.0", features = ["derive"] } # serde を追加
    ```
    編集したら、`cargo build` か `cargo run` を実行して、新しい依存関係をダウンロード＆コンパイルしておこう！

2.  **`lessons/my_rocket_app/src/main.rs` を開く**:
    いつものファイルにコードを追加していくよ！

3.  **JSONで返すデータ構造を定義する**:
    まず、どんなデータをJSONで返すか、Rustの構造体で定義しよう。
    今回は簡単なユーザー情報を返すことにするね。`serde` の `Serialize` トレイトを `derive` するのを忘れずに！これがJSONに変換するために必要なおまじないだよ✨
    ```rust
    use serde::Serialize; // serde::Serialize を use する

    #[derive(Serialize)] // Serialize を derive する
    struct User {
        id: u32,
        name: String,
        active: bool,
    }
    ```
    この構造体の定義は、`main` 関数とか他の関数の外、ファイルの上のほうに書くのが一般的だよ。

4.  **JSONを返すルートハンドラ関数を作る**:
    `/api/user` というパスにアクセスがあったときに、さっき定義した `User` 構造体のデータをJSON形式で返す関数を作ろう！
    関数名は `get_user` にして、戻り値の型を `rocket::serde::json::Json<User>` にするのがポイントだよ！これでRocketが自動的にJSONに変換して、適切なヘッダーを付けてくれるんだ。超便利！💖
    ```rust
    use rocket::serde::json::Json; // Json を use する

    #[get("/api/user")]
    fn get_user() -> Json<User> {
        let user = User {
            id: 1,
            name: "Roo".to_string(), // String 型にするために .to_string() が必要
            active: true,
        };
        Json(user) // User 構造体を Json でラップして返す
    }
    ```

5.  **`main` 関数を修正する**:
    `rocket()` 関数の中で、新しいルートハンドラ `get_user` も登録しよう！ `routes!` マクロにカンマで追加するだけ！
    ```rust
    #[launch]
    fn rocket() -> _ {
        rocket::build().mount("/", routes![index, hello, search, get_user]) // get_user を追加！
    }
    ```

6.  **実行して確認！**:
    ターミナルで `lessons/my_rocket_app` ディレクトリに移動して、`cargo run` コマンドを実行！
    サーバーが起動したら、Webブラウザか `curl` コマンドで `http://127.0.0.1:8000/api/user` にアクセスしてみよう！
    ブラウザだと、こんな感じのJSONが表示されるはずだよ（ブラウザによっては整形されて表示されるかも）。
    ```json
    {"id":1,"name":"Roo","active":true}
    ```
    `curl` コマンドを使う場合は、ターミナルでこう打ってみてね。
    ```bash
    curl http://127.0.0.1:8000/api/user
    ```
    同じようなJSONが表示されたら成功！🎉

## 完成イメージ (`lessons/my_rocket_app/src/main.rs`)

```rust
#[macro_use] extern crate rocket;

use rocket::serde::json::Json; // 追加
use serde::Serialize; // 追加

// 追加: データ構造の定義
#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/search?<term>")]
fn search(term: Option<String>) -> String {
    match term {
        Some(t) => format!("Searching for: {}", t),
        None => "Please provide a search term.".to_string(),
    }
}

// 追加: JSONを返すルートハンドラ
#[get("/api/user")]
fn get_user() -> Json<User> {
    let user = User {
        id: 1,
        name: "Roo".to_string(),
        active: true,
    };
    Json(user)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, search, get_user]) // get_user を routes! に追加
}
```

JSON APIが作れるようになると、一気に本格的なWebアプリ開発っぽくなるよね！✨
`serde` はRustでJSONとか他のデータ形式を扱うときの超定番ライブラリだから、覚えておくとめっちゃ役立つよ！

今回も分からなかったら、いつでも聞いてね！🙋‍♀️ 挑戦あるのみ！💪🔥