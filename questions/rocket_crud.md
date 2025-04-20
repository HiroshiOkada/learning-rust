# 課題: RocketでCRUD APIを実装しよう！

おつかれさま！メガネ君！✨
基本的なAPIの作り方はマスターしたね！すごい！👏

次は、もっと本格的なWebアプリケーションでよく使われる、**CRUD (Create, Read, Update, Delete)** 操作を実装してみよう！💪

今回は、ユーザー情報をメモリ上で管理して、それに対するCRUD操作を行うAPIエンドポイントを `/api/users` 以下に作っていくよ。

## データ構造と状態管理

まずは、ユーザー情報を保持するための構造体と、それを複数管理するための状態を定義しよう。

1.  **`User` 構造体の調整:**
    *   `src/main.rs` の `User` 構造体に `id` フィールドがすでにあることを確認しよう。（なければ追加してね！）
    *   `Serialize` に加えて `Deserialize` も `derive` しておこう。更新処理で使うかもしれないからね。あと、データを比較したりするために `PartialEq` と `Clone` もあると便利だよ。
    ```rust
    use serde::{Serialize, Deserialize}; // 確認

    #[derive(Serialize, Deserialize, Clone, PartialEq)] // Clone と PartialEq を追加
    struct User {
        id: u32,
        name: String,
        active: bool, // これはあってもなくてもOK
    }
    ```

2.  **状態管理の導入:**
    *   複数の `User` データをメモリ上で保持するために、`Mutex<Vec<User>>` を使った状態管理を導入しよう。`Mutex` は複数のリクエストから安全にデータにアクセスするために必要だよ。
    *   `main` 関数 (または `rocket()` 関数) で初期データを持つ `Mutex` を作成し、`manage` メソッドでRocketの状態として登録するよ。
    ```rust
    use std::sync::Mutex; // Mutex をインポート
    use rocket::State; // State をインポート

    // ダミーデータ（必要に応じて変更してね）
    let user_list: Mutex<Vec<User>> = Mutex::new(vec![
        User { id: 1, name: "Roo".to_string(), active: true },
        User { id: 2, name: "メガネ君".to_string(), active: true },
    ]);

    // rocket::build() の後に追加
    .manage(user_list)
    ```
    *   ルートハンドラ関数でこの状態にアクセスするには、引数に `&State<Mutex<Vec<User>>>` を追加するよ。

## やることリスト ✅

以下のCRUD操作に対応するルートハンドラを `src/main.rs` に実装し、`main` 関数 (または `rocket()` 関数) でマウントしよう。

1.  **Create (作成):**
    *   `POST /api/users` (既存の `create_user` を改造)
    *   リクエストボディで `NewUser` (または `User` の `id` 無し版) を受け取る。
    *   新しい `id` を採番して `User` を作成し、`Mutex` 内の `Vec` に追加する。
    *   作成された `User` 情報をJSONで返す。（ステータスコードは `201 Created` が望ましいけど、まずは `String` や `Json<User>` を返すだけでもOK！）
    *   **ヒント:** `Mutex` の中身を書き換えるには `.lock().unwrap()` でロックを取得する必要があるよ。新しいIDは、現在のリストの最大のID+1とか、簡単な方法でOK。

2.  **Read (読み取り):**
    *   `GET /api/users` (新規作成: `get_users`)
        *   `Mutex` 内のすべての `User` 情報を `Json<Vec<User>>` で返す。
    *   `GET /api/users/<id>` (新規作成: `get_user_by_id`)
        *   パスパラメータで指定された `id` に一致する `User` を `Mutex` 内の `Vec` から探す。
        *   見つかったら `Json<User>` で返す。
        *   見つからなかったら `None` やエラー（例: `NotFound` ステータス）を返す。（まずは `Option<Json<User>>` を返すことから試してみよう！）

3.  **Update (更新):**
    *   `PUT /api/users/<id>` (新規作成: `update_user`)
    *   パスパラメータで指定された `id` に一致する `User` を `Mutex` 内の `Vec` から探す。
    *   リクエストボディで更新後の `User` データ (または更新可能なフィールドのみを持つ構造体) をJSONで受け取る。
    *   見つかった `User` の情報を更新する。
    *   更新後の `User` 情報を `Json<User>` で返すか、成功メッセージを返す。
    *   見つからなかったら `None` やエラーを返す。

4.  **Delete (削除):**
    *   `DELETE /api/users/<id>` (新規作成: `delete_user`)
    *   パスパラメータで指定された `id` に一致する `User` を `Mutex` 内の `Vec` から探して削除する。
    *   成功したらステータスコード `204 No Content` を返すのが一般的だけど、まずは成功メッセージ (`String`) を返すだけでもOK！
    *   見つからなかったらエラーを返す。

## 実行と動作確認

*   `cargo run` でサーバーを起動しよう。
*   `curl` コマンドを使って、各エンドポイントが正しく動作するか確認しよう。
    *   `GET`: `curl http://127.0.0.1:8000/api/users` や `curl http://127.0.0.1:8000/api/users/1`
    *   `POST`: `curl -X POST -H "Content-Type: application/json" -d '{"name": "新しいユーザー"}' http://127.0.0.1:8000/api/users`
    *   `PUT`: `curl -X PUT -H "Content-Type: application/json" -d '{"id": 1, "name": "Rooちゃん", "active": false}' http://127.0.0.1:8000/api/users/1`
    *   `DELETE`: `curl -X DELETE http://127.0.0.1:8000/api/users/1`

## ヒント 💡

*   状態管理に `&State<Mutex<Vec<User>>>` を使うことを忘れずに！
*   `Mutex` を使うときは `.lock().unwrap()` でロックを取得しよう。スコープを抜けると自動でロックが解放されるよ。
*   `Vec` の操作（`push`, `find`, `position`, `remove` など）を活用しよう。
*   エラーハンドリングは最初はシンプルでOK。慣れてきたらRocketのレスポンダを使って適切なステータスコードを返すようにしてみよう。
*   `PUT` と `DELETE` メソッドに対応する `#[put(...)]` と `#[delete(...)]` アトリビュートを使おう。

ちょっと難易度上がるけど、これができたらWeb API開発の基本はバッチリだよ！💪
分からなかったら、いつでも聞いてね！応援してるよー！💖