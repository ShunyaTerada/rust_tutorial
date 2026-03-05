// 高度な関数とクロージャ
// 公式: https://doc.rust-jp.rs/book-ja/ch19-05-advanced-functions-and-closures.html

// 13章で学んだクロージャの応用編
// 関数ポインタとクロージャの返却について扱う


// ========================================
// 1. 関数ポインタ（fn 型）
// ========================================

// クロージャだけでなく、普通の関数も引数として渡せる
// その際に使うのが fn 型（小文字！）

// 💡 fn と Fn の違い:
//    fn  → 型（関数ポインタ型）。具体的な関数1つを指す
//    Fn  → トレイト（クロージャトレイト）。クロージャの振る舞いを抽象化
//    fn はトレイトではなく型なので、ジェネリクスのトレイト境界には使わない

fn add_one(x: i32) -> i32 {
    x + 1
}

// 引数 f の型が fn(i32) -> i32 → 「i32を受け取ってi32を返す関数ポインタ」
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn function_pointer_demo() {
    let answer = do_twice(add_one, 5);
    println!("do_twice(add_one, 5) = {}", answer);  // (5+1) + (5+1) = 12
}

// 💡 関数ポインタ fn は Fn, FnMut, FnOnce の3トレイト全てを実装している
//    → クロージャを期待する場所にも関数ポインタを渡せる
//    → 逆は不可（fn を期待する場所にクロージャは渡せないことがある）

// 実用例: map に関数名を直接渡す
fn map_with_function_demo() {
    let list_of_numbers = vec![1, 2, 3];

    // クロージャで書く場合
    let _strings1: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    // 関数名を直接渡す場合（フルパス記法）
    let strings2: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)   // Displayを実装する型は to_string が使える
        .collect();

    println!("関数名直接渡し: {:?}", strings2);
    // 💡 どちらも同じコードにコンパイルされるので好みの問題
}

// もう一つの実用例: enum のバリアントも関数ポインタとして使える
fn enum_as_function_demo() {
    // enum のバリアントは実はコンストラクタ関数
    // Status::Value(u32) は fn(u32) -> Status 型の関数として扱える
    #[derive(Debug)]
    enum Status {
        Value(u32),
        _Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..5)
        .map(Status::Value)    // Status::Value をコンストラクタ関数として渡す
        .collect();

    println!("enumバリアント: {:?}", list_of_statuses);
}

// C言語との相互運用（FFI）ではクロージャが存在しないので fn のみ受け入れるケースがある


// ========================================
// 2. クロージャを返却する
// ========================================

// クロージャはトレイト（Fn, FnMut, FnOnce）で表現される
// トレイトを直接戻り値の型にはできない（コンパイル時にサイズが分からないから）

// ❌ コンパイルエラーになる書き方:
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }
// → エラー: Sized が実装されていない

// ✅ 解決策: トレイトオブジェクト（Box<dyn Fn(...)>）を使う
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// 💡 なぜ Box が必要か？
//    クロージャはそれぞれ固有の匿名型を持つ（サイズがコンパイル時に不定）
//    Box でヒープに配置すれば、Box 自体は固定サイズのポインタになる
//    17章のトレイトオブジェクト（Box<dyn Draw> 等）と同じ考え方

fn returning_closure_demo() {
    let closure = returns_closure();
    println!("返却されたクロージャ: closure(5) = {}", closure(5));  // 6
}

// 実用的な例: 設定に応じて異なるクロージャを返す
fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n)   // move で n の所有権をクロージャ内にキャプチャ
}

fn practical_closure_demo() {
    let add_5 = make_adder(5);
    let add_10 = make_adder(10);
    println!("add_5(3) = {}", add_5(3));    // 8
    println!("add_10(3) = {}", add_10(3));  // 13
}


fn main() {
    println!("=== 1. 関数ポインタ ===");
    function_pointer_demo();
    map_with_function_demo();
    enum_as_function_demo();

    println!("\n=== 2. クロージャを返却する ===");
    returning_closure_demo();
    practical_closure_demo();
}
