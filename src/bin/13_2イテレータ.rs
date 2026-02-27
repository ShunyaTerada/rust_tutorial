// イテレータとは
    // 一連の要素に順番に何らかの作業を行うパターン
    // Rustにおいてlazy(怠惰)であり、生成、変換しただけでは何もしない
    // なので、どのように消費されるかを指定する必要がある

fn iter() {
    let v1 = vec![1, 2, 3];
    
    let v1_iter = v1.iter(); // これだけではなにもしない
        // 所有された値を返すイテレータを生成したいなら、into_iter()
        // 可変参照を繰り返したいなら、iter_mut

    for val in v1_iter { //イテレータの所有権をムーブしている
        println!("取得：{val}");
    }   
}


// イテレータはIteratorトレイトを実装している
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
//     // デフォルト実装のあるメソッドは省略
// }
    // type Item, Self::Item でこのトレイトとの関連型を定義している
    // このItem型がnextメソッドの戻り値の型に使われている
    // nextメソッドは1度にSomeに包まれたイテレータの1要素を返し、繰り返しが終わったら、Noneを返す
fn next_demo() {
    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter();

    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);
}
    //nextメソッドはイテレータの要素を一つ、消費する


// 消費アダプタ (consuming adaptors)
    // 内部でnextを呼ぶメソッド群。呼び出すとイテレータを使い切る
    // sum, collect, for_each, count, last, nth など
fn consuming_adaptors() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // nextを繰り返し呼んで合計する
    assert_eq!(total, 6);
    // sumはイテレータの所有権を奪うので、この後v1_iterは使えない
    println!("合計：{total}");
}


// イテレータアダプタ (iterator adaptors)
    // イテレータを消費せず、別のイテレータに変換するメソッド群
    // map, filter, take, skip, zip, enumerate, chain など
    // lazy なので、消費アダプタで消費しないと何もしない
fn iterator_adaptors() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // mapだけでは何もしない（警告が出る）
    // v1.iter().map(|x| x + 1);

    // collectで消費して初めてVecになる
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    println!("map結果：{:?}", v2);
}


// 環境をキャプチャするクロージャとfilter
    // filterはクロージャがtrueを返した要素だけを含む新しいイテレータを返す
    // クロージャは環境の変数をキャプチャできる（13-1で学んだ内容）

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter()で所有権を奪うイテレータを作成
    // filterのクロージャがshoe_size（環境の変数）をキャプチャしている
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn filter_demo() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
    println!("サイズ10の靴：{:?}", in_my_size);
}


fn main() {
    iter();
    next_demo();
    consuming_adaptors();
    iterator_adaptors();
    filter_demo();
}

