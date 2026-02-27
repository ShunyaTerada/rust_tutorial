// Vec<T> はヒープ上にメモリを確保し、同じ型の値を可変長で保持するコレクション

// スタック                    ヒープ
// ┌─────────────────┐       ┌───┬───┬───┐
// │ ptr  ──────────────────→│ 1 │ 2 │ 3 │
// │ len: 3          │       └───┴───┴───┘
// │ capacity: 3     │
// └─────────────────┘

fn main() {
    // Vec<T> はヒープ上にメモリを確保し、同じ型の値を可変長で保持するコレクション

    //作成
    let v: Vec<i32> = Vec::new();//ジェネリクスで型を指定している

    // vec! マクロで初期値付きベクタを作成（型推論される）
    let v2 = vec![1, 2, 3]; // 型推論で整数のデフォルトであるi32が採用されている
    println!("vec!マクロ: {:?}", v2);


    //更新
    let mut v3 = Vec::new();
    v3.push(5); // データからi32型だと推論されるので型注釈は不要
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("pushで追加: {:?}", v3);

    // pop で末尾から取り出し（Option<T>を返す）
    let last = v3.pop();
    println!("pop: {:?}, 残り: {:?}", last, v3);


    //取得
    let v4 = vec![1, 2, 3, 4, 5];

    // 方法1: 添え字記法（&v[index]）
    // → 範囲外アクセスでパニック！
    let third: &i32 = &v4[2]; 
    println!("添え字記法: 三つ目の要素は{}です", third);

    // 方法2: get メソッド → Option<&T> を返す
    // → 範囲外アクセスでも None を返すだけ（安全）
    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("getメソッド: 三つ目の要素は{}です", third),
        None => println!("三つ目の要素はありません"),
    }
    println!("\n======================");

    
    //借用規則に反した例
        //同一スコープ上で可変と不変な参照を同時に存在させられない
        //ベクタの場合は違うインデックスの要素同士でも上記の規則が適用される
    let mut v5 = vec![1, 2, 3, 4, 5];

    let _first = &v5[0];//最初の要素を不変参照

    v5.push(6);//最後の要素を変更（可変参照）
    //これ以降でprintln!("{first}");のように参照しようとするとエラーになる


    //値を順番に処理する
        //不変参照
    let v6 = vec![100, 32, 57];
    for i in &v6 {
        println!("不変参照：{i}");
    }
        //可変参照
    let mut v7 = vec![100, 32, 57];
    for i in &mut v7 {
        *i += 50; //参照外し演算子でポインタを追いかける
        println!("可変参照：{i}");
    }
    println!("\n======================");


    //enumを使って複数の型を保持する
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("複数型ベクタ：{:?}", row);
    println!("\n======================");
}
