// パターンとマッチング
// 公式: https://doc.rust-jp.rs/book-ja/ch18-00-patterns.html

// パターンはRust特有の強力な文法
    // 値の構造を分解したり、条件分岐をスマートに書くために使う


// パターンが使われる全ての場所
    // 1. match アーム        → match VALUE { PATTERN => EXPRESSION }
    // 2. if let 式           → if let PATTERN = VALUE
    // 3. while let 条件ループ
    // 4. for ループ          → for PATTERN in ITERATOR
    // 5. let 文              → いつも使っている let は実はパターン！
    // 6. 関数の引数          → fn foo(PATTERN: TYPE)
    //
    // 💡 つまり、letやforで毎回使っていた構文の正体がパターンだった


// 論駁可能性（Refutability）
    // パターンがマッチしない可能性があるかどうかで、コンパイラの扱いが変わる
    //
    // 論駁不可 (irrefutable): 絶対にマッチする
        // 例: let x = 5; の x（何にでもマッチする）
    // 論駁可能 (refutable): マッチしない可能性がある
        // 例: if let Some(x) = a_value の Some(x)（Noneだとマッチしない）
    //
    // 💡 let文やforループには「論駁不可」なパターンしか使えない
        // マッチしない可能性がある場合は if let を使う


// パターン記法のチートシート

struct Point { x: i32, y: i32 }
enum Message {
    Hello { id: i32 },
}

fn pattern_syntax() {
    // 1. 複数パターンのマッチ（|）
    let x = 1;
    match x {
        1 | 2 => println!("1か2です"),
        3 => println!("3です"),
        _ => println!("それ以外"),
    }

    // 2. 範囲のマッチ（..=）
        // 数値やchar型に使える
    let y = 5;
    match y {
        1..=5 => println!("1から5の間です"),
        _ => println!("それ以外"),
    }

    // 3. 構造体の分配（Destructuring）
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // aに0、bに7が入る
    let Point { x, y } = p;       // フィールド名と同じ変数名を付ける際の省略記法

    // 4. 値の一部を無視（_ や ..）
    let tuple = (1, 2, 3, 4, 5);
    let (first, .., last) = tuple; // 真ん中を無視して両端だけ変数に束縛する
    println!("最初: {}, 最後: {}", first, last);

    // 5. マッチガード（パターンのあとに if で追加の条件式を書く）
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("5未満の数: {}", x),
        Some(x) => println!("5以上の数: {}", x),
        None => (),
    }

    // 6. @ バインディング
        // パターンにマッチした値を、その場で変数にも束縛する
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            // 変数 id_variable に値(5)を入れつつ、3..=7の範囲に収まっているかテストする
            println!("範囲内のid: {}", id_variable)
        },
        Message::Hello { id } => println!("その他のid: {}", id),
    }
}


// _ と _x の違い（所有権の挙動）
    // _x → 名前がついた変数。値を束縛する（所有権を奪う）。警告を黙らせるだけ
    // _  → ワイルドカード。値を束縛しない（所有権を奪わない）
    //
    // 💡 Copy型（i32など）では問題にならない。差が出るのは String, Vec など非Copy型のとき
fn underscore_patterns() {
    // _s は変数なので、String の所有権が s から _s にムーブされる
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("文字列が見つかりました");
    }
    // println!("{:?}", s);  // エラー！ s はもう使えない（_s にムーブ済み）

    // _ はワイルドカードなので、所有権のムーブが起きない
    let s2 = Some(String::from("Hello!"));
    if let Some(_) = s2 {
        println!("文字列が見つかりました");
    }
    println!("{:?}", s2);  // OK！ s2 はまだ生きている
}


// パターンにおける & の意味
    // 式（右辺）の & → 参照を「作る」    例: let r = &x;
    // パターン（左辺）の & → 参照を「剥がす」 例: let &x = r;
    //
    // 💡 「パターンは組み立ての逆」
    //    Point { x, y } で作ったものは、パターンの Point { x, y } で壊せる
    //    & で作ったものは、パターンの & で壊せる
    //    結果的に * での参照外しと同じだが、道具が違う（式 vs パターン）
fn ref_in_pattern() {
    let v = vec![1, 2, 3];

    // iter() は &i32 を返す
    // パターン &x で参照を分解して i32 を取り出す
    for &x in v.iter() {
        println!("{}", x);  // x は i32
    }
}


// ref と ref mut でパターンに参照を生成する
    // パターンの中で変数に値を束縛すると、通常は所有権がムーブされる
    // 所有権を奪わずに「借用」したいとき、ref を使う
    //
    // 💡 なぜ & ではダメなのか？
    //    パターンでの & は「既存の参照にマッチして剥がす」意味（上で学んだやつ）
    //    参照を「生成する」意味にはならない
    //    だから新しいキーワード ref が必要
fn ref_and_ref_mut() {
    // ref なし → 所有権がムーブされてしまう
    let robot_name = Some(String::from("Bors"));
    match &robot_name {
        // ref を使って、所有権を奪わずに借用する
        Some(name) => println!("名前が見つかりました: {}", name),  // name は &String
        None => (),
    }
    println!("robot_name: {:?}", robot_name);  // OK！ 所有権はムーブされていない

    // ref mut → 可変参照を生成する
        // &mut ではなく ref mut を使う理由も同じ
        // パターンでの &mut は「既存の可変参照にマッチ」する意味だから
    let mut robot_name2 = Some(String::from("Bors"));
    match robot_name2 {
        Some(ref mut name) => *name = String::from("Another name"),  // 可変参照なので * で参照外しして代入
        None => (),
    }
    println!("robot_name2: {:?}", robot_name2);  // Some("Another name")
}


fn main() {
    pattern_syntax();
    underscore_patterns();
    ref_in_pattern();
    ref_and_ref_mut();
    println!("18章のパターンとマッチング記法まとめました。");
}
