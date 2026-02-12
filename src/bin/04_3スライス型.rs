fn main(){
    println!("=============");

    normal_reference();
    println!("\n=============");

    str_slice();
    println!("\n=============");
}

//通常の部分参照

//Stringのsと添え字のwordが同期していない
fn normal_reference(){
    let mut s = String::from("hello normal");
    let word = first_word(&s);

    println!("{word}");


    s.clear();
    // wordはまだ値5を保持しているが、もうこの値を正しい意味で使用できる文字列は存在しない。
}


fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();//バイトに変換

    //空白を表すバイトを検索してその位置を返す
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    //空白以外は文字列の長さを返す
    s.len()
}

//スライス型

//文字列スライス
fn _slice_demo(){
    let s = String::from("hello slice");

    let _hello = &s[0..5];
    let _world = &s[6..11];
}

//スライスデータ構造は、開始地点とスライスの長さを保持している

fn str_slice(){
    let mut s = String::from("hello slice");
    let word = first_word_slice(&s);

    println!("{word}");

    s.clear();
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
    if item == b' ' {
        return &s[0..i];    
    }
}

    &s[..]
}

//文字列リテラルは不変参照
fn _str_literal(){
    let _s = "Hello, literal!";//カーソルを合わせると&strとわかる
}

//整数スライス
fn _integer_slice(){
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];//&[i32]という型

    assert_eq!(slice, &[2, 3]);
}