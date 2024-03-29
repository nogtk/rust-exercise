# 2.1章 数当てゲーム
- Rustが自動的にデフォルトで読み込むものはプレリュードと呼ばれ、それ以外はuseしないといけない
    - 標準入出力とやり取りするインターフェースが必要なら`use std::io;` する必要がある
    - プレリュード: おしゃれ。
```rust
let foo = bar;
```
- fooという束縛を作り、それをbarに束縛する、ということを表現している
    - 他の多くの言語では「変数」と呼ばれるもの。Rustでは束縛変数という
- Rustではletで宣言した束縛はデフォルトでイミュータブル。
    - ミュータブルな束縛を定義するためにmutを使う ( Ex. let mut foo = bar )

```rust
String::new()
```
- String型のインスタンスを生成しているわけではない
- ::new()は特定の型の「関連関数」(ここはよくわかってない)
    - スタティックメソッド、なんて名前で呼ばれているらしい
```rust
io::stdin().read_line(&mut guess).expect("hoge");
```
- io::stdin()は標準入出力をハンドルする
- guessという参照もデフォルトではイミュータブルなので、&mutをつけてミュータブルとして参照している
    - read_lineで取得したユーザの入力文字列を取得し、guessに代入するためにはミュータブルである必要がある

```rust
println!("hoge {}", hoge);
```
- {}はプレースホルダ
    - 複数の変数を入れたいなら「{}」も複数になる
    
 - Rustコードのパッケージは「クレート」と呼ばれる
    - Cargoプロジェクトで管理をしているなら、Cargo.tomlの[dependency]欄にクレートを記述することで依存関係を考慮してパッケージのダウンロードをしてくれる
    
- クレートはファイル内でextern use として指定することでインクルードする
- インクルードしたメソッドは、トレイトと呼ばれる空間で定義されているので、トレイトをスコープ内に取り込む必要がある ( Ex. rand::Rng)

- 値の大小を比較するときにはOrderingを使う
    - OrderingはenumでLess, Equal, Greaterという3つのバリアントを保持する

- Rustは型推論を持っているため、束縛される対象がString::new()とあれば、コンパイラはString型である、と理解をしてくれる
    - 数値に関しては、i32やu32,f64など、様々な型が用意されている
    - 指定がなければRustはデフォルトでi32として解釈する
    - 数値と文字列を単に比較しようとするとコンパイラはエラーを吐く
 
 - Rustでは同じ変数を再定義することができる( シャドーイング )
    - 型の変換時などに用いれられる
    
- matchがif文のような役割を果たしている
    - 返り値のenumに対して等しいか、大きいか小さいかなどを判定して、それに対するアクションを定義している

#2.2 食事する哲学者
- `struct`で構造体を定義する
    - メンバ名と型を宣言する
    - String型と&str型の違いは不明
- implブロックで構造体に関する定義を与える
```rust
impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
}
```
なら、Philospherに対してnewという"関連関数"を定義する
newではPhilosopherのインスタンスを返す
- rubyと同様、最後に評価した値が返り値となる
- new()という名前に縛りはないが、構造体からインスタンスを生成する際に慣例的に用いられる事が多い
- 可変長配列にはVec<T>を用いる
- Rustではメソッドは明示的なselfパラメータを取る
    - selfパラメータを取らないものは関連関数 ( 上のnewメソッドは関連関数 )
- Mutex: 並行処理を制御するための機構
    - 同時アクセスできるスレッドを単一にする
    - 今回の例だと各フォークにアクセスできるのは一人のみのためMutexを用いる
- Arc = アトミック参照カウント
    - 各スレッドからリソースを共有するために必要となる
    - 共有するときは参照カウントを増やし、各スレッドの終了時にはカウントを減らす　
    
# 2.3 Rust Inside Other Languages
- 高級言語の実行速度の遅さを補うためによくCで書いた実装を他の言語から呼び出して利用することがある
    - この技術はForeign Function Interface (FFI) と呼ばれる
## The Problem
- オブジェクト指向なプログラミング言語はGCを利用している
    - それで最適化されることもあるが、静的にメモリをアロケートしたい場合がある
- 多くの言語はGIL(Global Interpreter Lock) を持っている
    - これは並行処理に制限を与えている
## FFI
- fn の前に pub とつければモジュールの外から呼び出しが可能になる
- extern はCから呼び出す際に必要な枕詞になる