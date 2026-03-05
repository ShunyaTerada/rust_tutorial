// マクロ
// 公式: https://doc.rust-jp.rs/book-ja/ch19-06-macros.html

// マクロ = 「コードを書くコード」（メタプログラミング）
// println!(), vec![], derive(...) など、既にたくさん使ってきた


// ========================================
// マクロと関数の違い
// ========================================

// 関数と似ているが、マクロには関数にない力がある:
    // 1. 可変長の引数を取れる
    //    println!("hello")        → 1引数
    //    println!("{} {}", a, b)  → 3引数
    //    関数だと引数の数を宣言しなければならないが、マクロは自由
    //
    // 2. コンパイル前に展開される
    //    マクロはコンパイラがコードの意味を解釈する前に処理される
    //    だからトレイトの実装を生成できる（関数は実行時に呼ばれるので不可能）
    //
    // 3. マクロは呼び出す前に定義（またはスコープに導入）が必要
    //    関数はどこに定義してもOKだが、マクロは順序が重要

// デメリット:
    // マクロの定義は「Rustコードを生成するRustコード」なので複雑になりがち
    // 関数よりも読みにくく、管理しづらい


// ========================================
// 1. 宣言的マクロ（Declarative Macros）: macro_rules!
// ========================================

// Rustで最もよく使われるマクロの形
// match式に似たパターンマッチングで、コードをコードに置き換える

// vec! マクロの簡略版を見てみよう:
//
//   #[macro_export]
//   macro_rules! vec {
//       ( $( $x:expr ),* ) => {
//           {
//               let mut temp_vec = Vec::new();
//               $(
//                   temp_vec.push($x);
//               )*
//               temp_vec
//           }
//       };
//   }

// パターンの読み方:
    // $( $x:expr ),*
    //   $x:expr  → 任意のRust式にマッチし、$x という名前で捕捉する
    //   $( ... ),*  → カンマ区切りで0個以上の繰り返しにマッチ
    //
    // 展開部分:
    //   $( temp_vec.push($x); )*
    //   → マッチした各 $x に対して push 文を生成する

// 💡 vec![1, 2, 3] と書くと、コンパイル時に以下のコードに展開される:
    // {
    //     let mut temp_vec = Vec::new();
    //     temp_vec.push(1);
    //     temp_vec.push(2);
    //     temp_vec.push(3);
    //     temp_vec
    // }

// #[macro_export] を付けると、クレートがスコープに持ち込まれたとき
// このマクロが利用可能になる（付けないとクレート外では使えない）

// 自分で宣言的マクロを作ってみる
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn declarative_macro_demo() {
    say_hello!();               // Hello!
    say_hello!("Rust");         // Hello, Rust!

    // vec! は標準マクロなのでそのまま使える
    let v = vec![1, 2, 3];
    println!("vec!で生成: {:?}", v);
}


// ========================================
// 2. 手続き的マクロ（Procedural Macros）
// ========================================

// 宣言的マクロがパターンマッチでコードを置き換えるのに対して、
// 手続き的マクロは「Rustコードを入力として受け取り、加工して、新しいコードを出力」する

// 手続き的マクロには3種類ある:
    // a. カスタム derive マクロ
    // b. 属性風マクロ（Attribute-like macros）
    // c. 関数風マクロ（Function-like macros）

// 💡 手続き的マクロは専用のクレート（proc-macro クレート）に定義する必要がある
//    これは技術的な制約であり、将来的に緩和される可能性がある

// 手続き的マクロの基本形:
//   use proc_macro;
//
//   #[some_attribute]
//   pub fn some_name(input: TokenStream) -> TokenStream {
//       // input を加工して新しい TokenStream を返す
//   }
//
// TokenStream: Rustのソースコードをトークン列として表現した型
//   ソースコード → TokenStream(入力) → マクロ処理 → TokenStream(出力) → コンパイル続行


// ----------------------------------------
// 2a. カスタム derive マクロ
// ----------------------------------------

// #[derive(Debug)] や #[derive(Clone)] のように
// #[derive(MyTrait)] でトレイトの実装コードを自動生成する

// 使い方（使う側）:
//   use hello_macro::HelloMacro;
//   use hello_macro_derive::HelloMacro;   ← derive マクロのクレート
//
//   #[derive(HelloMacro)]
//   struct Pancakes;
//
//   fn main() {
//       Pancakes::hello_macro();  // Hello, Macro! 私の名前はPancakesです！
//   }

// 定義する側（hello_macro_derive クレート内）:
//   #[proc_macro_derive(HelloMacro)]
//   pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
//       // input（構造体定義のソース）を解析して、
//       // HelloMacro トレイトの実装コードを生成して返す
//   }

// 💡 derive マクロを作る際の主要な依存クレート:
//    syn      → TokenStream を解析してRustの構文木（AST）に変換
//    quote    → Rustコードを TokenStream に変換（テンプレートエンジンのような役割）
//    proc_macro → Rust組み込み。TokenStream 型を提供


// ----------------------------------------
// 2b. 属性風マクロ（Attribute-like macros）
// ----------------------------------------

// derive の代わりに、任意の属性を作成できる
// derive は構造体/enumにしか使えないが、属性風マクロは関数などにも使える

// 使い方:
//   #[route(GET, "/")]
//   fn index() { ... }

// 定義:
//   #[proc_macro_attribute]
//   pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
//       // attr: 属性の引数部分 → GET, "/"
//       // item: 属性が付けられた要素 → fn index() { ... }
//   }

// 💡 Web フレームワーク（Actix, Axumなど）でよく見る #[get("/")] や #[post("/api")]
//    はまさにこの仕組み


// ----------------------------------------
// 2c. 関数風マクロ（Function-like macros）
// ----------------------------------------

// 関数呼び出しのように見える手続き的マクロ
// macro_rules! よりも複雑な処理ができる

// 使い方:
//   let sql = sql!(SELECT * FROM posts WHERE id=1);

// 定義:
//   #[proc_macro]
//   pub fn sql(input: TokenStream) -> TokenStream {
//       // input の中身（SQL文）をパースして構文チェックし、
//       // 正しいRustコードとして TokenStream を返す
//   }

// 💡 macro_rules! との違い:
//    macro_rules! はパターンマッチ（match風）でしか定義できない
//    関数風マクロは任意のRustコードで処理できるのでより柔軟


// ========================================
// まとめ: マクロの種類と使い分け
// ========================================

// | 種類                | 記法              | 主な用途                     |
// |---------------------|-------------------|------------------------------|
// | 宣言的マクロ        | macro_rules!      | vec![] 等の汎用コード生成    |
// | カスタム derive      | #[derive(Trait)]  | トレイトの自動実装           |
// | 属性風マクロ         | #[attr(...)]      | 関数等へのメタデータ付与     |
// | 関数風マクロ         | name!(...)        | 複雑なDSLやパース処理        |
//
// 💡 初心者が「作る」必要があるのは稀。まずは「使う側」として仕組みを理解しよう
//    derive(Debug), derive(Clone), vec![], println![] などは毎日使うマクロ


fn main() {
    println!("=== 1. 宣言的マクロ ===");
    declarative_macro_demo();

    println!("\n=== 2. 手続き的マクロ ===");
    println!("手続き的マクロは専用クレートで定義するため、ここではコメントで解説しています。");
    println!("derive(Debug) や derive(Clone) を使うたびに、裏で手続き的マクロが動いています。");

    // derive マクロの使用例（日常的に使っているもの）
    #[derive(Debug, Clone, PartialEq)]
    struct Example {
        name: String,
        value: i32,
    }

    let a = Example { name: String::from("test"), value: 42 };
    let b = a.clone();  // Clone が derive で自動実装されている
    println!("Debug表示: {:?}", a);    // Debug が derive で自動実装されている
    println!("同値比較: {}", a == b);  // PartialEq が derive で自動実装されている
}
