# 課題: POSTリクエストでJSONデータを受け取ろう！

おつかれさま！メガネ君！✨
前回はサーバーからJSONを返すAPIを作ったけど、今度はクライアントから送られてくるJSONデータを受け取るAPIを作ってみよう！💪

HTTPメソッドの `POST` を使って、新しいユーザー情報を受け付けるAPIエンドポイント `/api/users` を作成します。

## やることリスト ✅

1.  **`serde` の依存関係を確認:**
    *   `Cargo.toml` を開いて、`serde` の `features` に `"derive"` が含まれていることを確認しよう。（前回の課題で追加済みのはずだけど、念のためチェック！✅）
    ```toml
    [dependencies]
    rocket = { version = "0.5.0", features = ["json"] }
    serde = { version = "1.0", features = ["derive"] }
    ```

2.  **受け取るデータ用の構造体定義:**
    *   `src/main.rs` に、クライアントから受け取るJSONデータの構造を表す `NewUser` 構造体を定義しよう。
    *   `serde::Deserialize` を `derive` するのを忘れずに！これでJSONからRustの構造体に変換できるようになるよ。
    ```rust
    use serde::Deserialize; // これを追加

    #[derive(Deserialize)] // Deserialize を derive する
    struct NewUser {
        name: String,
        // 他に必要なフィールドがあれば追加してもOK！
    }
    ```

3.  **POSTルートハンドラの作成:**
    *   `src/main.rs` に、`/api/users` パスへのPOSTリクエストを処理する `create_user` 関数を作成しよう。
    *   `#[post("/api/users", data = "<user_data>")]` アトリビュートを使って、POSTリクエストであること、パス、そしてリクエストボディのデータを受け取る変数名 (`user_data`) を指定するよ。
    *   関数の引数で `Json<NewUser>` 型のデータを受け取るようにしよう。これでRocketが自動的にリクエストボディのJSONを `NewUser` 構造体にデシリアライズしてくれるんだ！超便利！✨
    *   今回はシンプルに、受け取ったユーザーの名前を含む成功メッセージを返してみよう。
    ```rust
    use rocket::serde::json::Json; // これも必要

    #[post("/api/users", data = "<user_data>")]
    fn create_user(user_data: Json<NewUser>) -> String {
        // user_data は Json<NewUser> 型なので、中の NewUser を使うには .into_inner() するか、
        // フィールドにアクセスするなら user_data.name のように書けるよ
        format!("User '{}' created successfully!", user_data.name)
    }
    ```

4.  **ルートのマウント:**
    *   `main` 関数 (または `rocket()` 関数) の `rocket::build().mount()` に、新しく作った `create_user` ルートを追加しよう。
    ```rust
    #[launch]
    fn rocket() -> _ {
        rocket::build().mount("/", routes![index, hello, search, get_user, create_user]) // create_user を追加！
    }
    ```

5.  **実行と動作確認:**
    *   `cargo run` でサーバーを起動しよう。
    *   ターミナルから `curl` コマンドを使って、POSTリクエストを送ってみよう！
        *   `-X POST` でPOSTメソッドを指定
        *   `-H "Content-Type: application/json"` で送るデータがJSON形式であることを示すヘッダーを追加
        *   `-d '{ "name": "メガネ君" }'` で送信するJSONデータを指定
    ```bash
    curl -X POST http://127.0.0.1:8000/api/users \
         -H "Content-Type: application/json" \
         -d '{ "name": "メガネ君" }'
    ```
    *   サーバーから `User 'メガネ君' created successfully!` のようなメッセージが返ってきたら成功！🎉

## ヒント 💡

*   `Deserialize` は、JSONなどの外部データをRustのデータ構造に変換（デシリアライズ）するために使うよ。`Serialize` の逆だね！
*   `#[post(...)]` アトリビュートの `data = "<変数名>"` で、リクエストボディのデータを関数の引数に紐付けるよ。
*   `Json<T>` 型は、リクエストボディのJSONを自動で `T` 型にデシリアライズしてくれる便利な型だよ。

分からなかったら、いつでも聞いてね！応援してるよー！💖