# 勉強したことをまとめていくよ
## 参考
[プログラミング言語Rust](https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/README.html)
# 2章 はじめる
## 知ったこと
- ! がついているとrustではマクロ扱いとなる
    - ついていなければ関数
    - マクロと関数の違いについては不明
- Cargoとはrustのビルドシステムでもありパッケージマネージャでもある
- Cargoはソースファイルをsrcディレクトリに存在することを要求する (!)
- Cargo.toml(Cは大文字)にCargoの設定を記述する
    - TOML: Tom's Obvious, Minimal Language
    - Cargoの設定フォーマットとして使われる
- `cargo build` でプロジェクトをビルドし、`cargo run`でプロジェクトを実行する
- `cargo run` では変更点の差分のみをコンパイルして実行してくれる
    - Makefileみたいな
- `cargo new`ってやるとtomlファイルとsrc配下にソースファイルが配置されたプロジェクトを生成してくれる