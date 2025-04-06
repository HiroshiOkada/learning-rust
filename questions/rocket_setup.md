# 課題: Rocket プロジェクトをセットアップしよう！🚀

いよいよ Web アプリ開発！まずは Rocket を使うための準備をしよう！

## やってみよう！

1.  **新しいプロジェクトを作る**:
    *   ターミナルを開いて、`learning-rust` ディレクトリの中に `my_rocket_app` という名前で新しい Rust プロジェクトを作ってね。
    ```bash
    # learning-rust ディレクトリにいることを確認！
    cargo new my_rocket_app
    ```

2.  **プロジェクトディレクトリに移動する**:
    ```bash
    cd my_rocket_app
    ```

3.  **`Cargo.toml` に Rocket を追加する**:
    *   `my_rocket_app` ディレクトリの中にある `Cargo.toml` ファイルを開く。
    *   `[dependencies]` セクションに以下の行を追加する。
    ```toml
    [dependencies]
    rocket = "0.5.0"
    ```

4.  **Nightly Toolchain を使う設定**:
    *   `my_rocket_app` ディレクトリの直下に `rust-toolchain.toml` という名前の新しいファイルを作る。
    *   そのファイルに以下の内容を書き込む。
    ```toml
    [toolchain]
    channel = "nightly"
    ```
    *   **もし Nightly Toolchain がインストールされてなかったら**: ターミナルで `rustup toolchain install nightly` を実行してね！

---

ここまでできたら、教えてね！💖
次は、いよいよ最初の Rocket コードを書いて、Web サーバーを動かしてみるよ！✨ ワクワクするね！😆