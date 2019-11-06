fn main() {
    let number = 3;
    if number < 5 {
        println!("hoge");
    } else {
        println!("Fuga");
    }

    let flag = true;
    let y = if flag {
        6
    } else {
        8
    };

    println!("y = {}", y);

    let a = [10, 20, 30, 40];
    for i in a.iter() {
        println!("the value is {}", i)
    }
}
