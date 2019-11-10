fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    // この後sをclearするなどして更新すると word = 5 という値が意味をなさなくなる
    // 論理的に誤っているわけではないのでコンパイルエラーにならない
    // wordを使ってsの文字列を参照しようとしたときに初めて気がつく


    let mut s1 = &s;
    let word_slice = first_word_slice(s1);
    // スライスを使えば借用扱いになるので、元の文字列s1を変更しようとすればコンパイルエラーが吐かれる
    // 不変な参照があるときに借用元の値を変えることはできない

    println!("{}", word_slice);

    let word_slice2 = first_word_slice("abcabc abc");

    println!("{}", word_slice2);
}

// 最初の単語を見つける　
// 空白文字の最初のインデックスを返す
// 空白がなければ文字列全体の長さを返す
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
