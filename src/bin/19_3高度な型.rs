// 高度な型
// 公式: https://doc.rust-jp.rs/book-ja/ch19-04-advanced-types.html

// Rustの型システムをもっと深く使いこなすためのテクニック集
// ニュータイプパターン、型エイリアス、Never型、動的サイズ型を扱う


// ========================================
// 1. ニュータイプパターンの応用（型安全性と抽象化）
// ========================================

// 19_2 では「孤児ルールの回避」としてニュータイプを学んだが、他にも使い道がある

// 用途1: 単位の区別（型安全性）
    // u32 を直接使うと「年齢」と「ID」を取り違えるかもしれない
    // ニュータイプで包むと、コンパイラが型レベルで区別してくれる
struct Age(u32);
struct Id(u32);

fn print_age(age: &Age) {
    println!("年齢: {}", age.0);
}

// print_age(Id(123)) → コンパイルエラー！ Age と Id は別の型

// 用途2: 内部実装の隠蔽
    // struct People(HashMap<i32, String>) のようにして
    // 外部には People 型として公開し、内部のHashMapを直接触らせない
    // → カプセル化の実現

fn newtype_safety_demo() {
    let age = Age(25);
    let _id = Id(1001);
    print_age(&age);
    // print_age(&_id);  // エラー！型が違う
}


// ========================================
// 2. 型エイリアス（Type Alias）
// ========================================

// type キーワードで既存の型に別名をつける
// 💡 ニュータイプと違って「新しい型」は作られない。あくまで既存の型の別名

type Kilometers = i32;

fn type_alias_demo() {
    let x: i32 = 5;
    let y: Kilometers = 10;

    // Kilometers は i32 の別名なので、そのまま計算できる
    println!("x + y = {}", x + y);  // 15
}

// 💡 型エイリアスの主な使い道: 長い型を短く書く
    // 例えば Result<T, E> を使うとき、E がいつも同じなら型エイリアスが便利

type Result<T> = std::result::Result<T, std::io::Error>;
// これでこのモジュール内では Result<i32> と書くだけで
// std::result::Result<i32, std::io::Error> の意味になる

// 標準ライブラリの std::io モジュールもまさにこのパターンを使っている
    // io::Result<T> は Result<T, io::Error> のエイリアス

// もっと極端な例: Box<dyn Fn() + Send + 'static> のような長い型も
    // type Thunk = Box<dyn Fn() + Send + 'static>; で短くできる

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(_f: Thunk) {
    // 引数の型がスッキリ！
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("こんにちは！"))
}

fn type_alias_practical_demo() {
    let f = returns_long_type();
    takes_long_type(f);
}


// ========================================
// 3. Never型（!）
// ========================================

// ! は「絶対に値を返さない」ことを型レベルで表現する特殊な型
// 正式には「空型（empty type）」や「never型」と呼ばれる

// 💡 実はもう使っていた！以下の場面で暗黙的に使われている:

// 場面1: match の中の continue
    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,          // u32 を返す
    //     Err(_) => continue,      // continue の戻り値は ! 型
    // };
    // continue は値を返さないので ! 型。! は任意の型に変換できるため、
    // match 全体の型が u32 に統一される

// 場面2: panic! マクロ
    // panic! の戻り値は ! 型
    // → match のアーム内で panic! しても他のアームの型と矛盾しない

// 場面3: loop（breakしない場合）
    // loop { ... } の式全体の型は ! 型（永遠に戻らないから）

// 💡 ! 型は「任意の型に型強制できる」特殊な性質を持つ
//    これにより、他の式と型が矛盾せずに組み合わせられる

fn never_type_demo() {
    // unwrap の内部実装も ! を使っている:
    // match self {
    //     Some(val) => val,
    //     None => panic!("..."),   ← panic! は ! 型なので val の型と矛盾しない
    // }

    let val: u32 = Some(42).unwrap();
    println!("unwrapの結果: {}", val);
}


// ========================================
// 4. 動的サイズ決定型（DST）と Sized トレイト
// ========================================

// Rustはコンパイル時に全ての型のサイズを知る必要がある（スタックに積むため）
// でも、コンパイル時にサイズが分からない型がある → 動的サイズ決定型（DST）

// 代表例: str（文字列スライスの中身）
    // &str は使えるが、str 単体は使えない
    // なぜなら str は実行時まで長さが分からないから
    //
    // &str → ポインタ(8バイト) + 長さ(8バイト) = 16バイト で固定サイズ！
    // これが「ファットポインタ」と呼ばれる仕組み

// 💡 DSTの黄金律:
//    動的サイズの型は、必ず何らかのポインタの背後に置く
//    &str, Box<str>, Rc<str> など

// もう一つの代表例: dyn Trait（トレイトオブジェクト）
    // Box<dyn Draw> の dyn Draw もサイズ不明の型
    // だからポインタ(Box, &, Rc)で包む必要がある

// Sized トレイト
    // コンパイル時にサイズが判明する全ての型に自動実装される特殊なトレイト
    //
    // ジェネリック関数は暗黙に Sized 制約を持つ:
    //    fn generic<T>(t: T) → 実際には fn generic<T: Sized>(t: T) と同じ
    //
    // DSTも受け入れたい場合は ?Sized で制約を緩める:
    //    fn generic<T: ?Sized>(t: &T) → T は Sized かもしれないし、そうでないかもしれない
    //    ただし引数は &T にしなければならない（サイズ不定だからポインタ経由）

fn sized_demo() {
    // &str は str へのファットポインタ（ポインタ + 長さ）
    let s: &str = "こんにちは";
    println!("文字列: {}", s);
    println!("&str のサイズ: {} バイト", std::mem::size_of::<&str>());  // 16バイト（64bit環境）

    // Box<str> も可能
    let boxed: Box<str> = "hello".into();
    println!("Box<str>: {}", boxed);
}

// ?Sized の具体例
fn print_it<T: ?Sized + std::fmt::Display>(val: &T) {
    // T は Sized かもしれないし、DSTかもしれない
    // どちらにしても &T（参照）で受け取るので大丈夫
    println!("値: {}", val);
}

fn question_sized_demo() {
    print_it(&42);          // T = i32 (Sized)
    print_it("hello");      // T = str (DST) → &str として渡される
}


fn main() {
    println!("=== 1. ニュータイプの応用 ===");
    newtype_safety_demo();

    println!("\n=== 2. 型エイリアス ===");
    type_alias_demo();
    type_alias_practical_demo();

    println!("\n=== 3. Never型 ===");
    never_type_demo();

    println!("\n=== 4. 動的サイズ型と Sized ===");
    sized_demo();
    question_sized_demo();
}
