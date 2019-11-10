fn main() {
    // String型文字列の定義
    let mut s = String::from("hello");
    println!("{}", s);
    // 文字列リテラルを結合させる
    s.push_str(", world!");
    println!("{}", s);

    let s = String::from("hello");

    // 関数呼び出しでもムーブは発生する
    takes_ownership(s);

    // ここでsに参照しようとするとコンパイルエラー

    let x = 5;

    // 整数リテラルではムーブは発生しない ( スタックで管理されているデータだから )
    makes_copy(x);

    // ここでxに参照してもコンパイラエラーは発生しない

    // gives_ownershipは戻り値をs1にムーブする
    let s1 = gives_ownership();
    // s2がスコープに入る
    let s2 = String::from("hello");
    // s2がtakes_and_gives_backにムーブし、takes_and_gives_backからs3にムーブされる
    let s3 = takes_and_gives_back(s2);

} // s,s2はドロップ対象外、x,s1,s3はドロップ対象、

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hoge");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
