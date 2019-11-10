fn main() {
    let s1 = String::from("hello");
    // &s1という記述で所有権をcalculate_lengthに渡すことなくs1の情報を参照している
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    // これはコンパイルエラー
    // 参照も変数と同様、デフォルトでイミュータブル
    // change(&s)

    // これはエラーでない
    // mutで可変な参照を宣言できる
    change(&mut s);

    let s2 = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");

    // ダングリングポインタの生成はコンパイラレベルでエラーを吐いてくれる
    // &s;
    s
}
