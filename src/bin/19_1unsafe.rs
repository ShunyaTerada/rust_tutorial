// Unsafe Rust
// 公式: https://doc.rust-jp.rs/book-ja/ch19-01-unsafe-rust.html

// Rustのコンパイラは安全性チェック（借用チェッカー等）で守ってくれるが、
// 「プログラマには安全だとわかるけどコンパイラには判断できない」場面がある
// そういうときに unsafe ブロックを使って、コンパイラに
// 「ここは僕が責任を持つ」と宣言する

// 💡 unsafeにしても借用チェッカーが無効になるわけではない！
//    unsafeで追加で解禁されるのは以下の4つだけ:
//    1. 生ポインタの参照外し
//    2. unsafeな関数やメソッドの呼び出し
//    3. 可変なstatic変数へのアクセス・変更
//    4. unsafeなトレイトの実装

// 💡 unsafeブロック ≠ 危険なコード
//    「このブロック内のメモリ安全性はプログラマが保証する」という契約書のようなもの
//    バグが起きたときに調査範囲を絞れるので、unsafeブロックは小さくするのが鉄則


// ========================================
// 1. 生ポインタ（Raw Pointer）を参照外しする
// ========================================

// 生ポインタとは？
    // 参照（&T, &mut T）に似ているが、Rustの安全保障を受けない「素」のポインタ
    // *const T → 不変の生ポインタ（読み取り専用）
    // *mut T   → 可変の生ポインタ（書き込み可能）
    // ※ ここの * は参照外しではなく、型名の一部

// 参照との違い:
    // - 同じ場所への不変・可変ポインタが同時に存在できる（借用規則を無視）
    // - 有効なメモリを指している保証がない
    // - nullの可能性がある
    // - 自動的な片付け（Drop）は実装されていない

fn raw_pointers() {
    let mut num = 5;

    // 参照から生ポインタを生成する（as でキャスト）
    // 💡 生ポインタの「生成」自体はsafeコードで可能
    //    「参照外し」がunsafeになるだけ
    let r1 = &num as *const i32;     // 不変の生ポインタ
    let r2 = &mut num as *mut i32;   // 可変の生ポインタ

    // 参照外しするにはunsafeブロックが必要
    unsafe {
        println!("r1の値: {}", *r1);   // 参照外しして値を読む
        println!("r2の値: {}", *r2);
    }

    // ⚠️ 任意のメモリアドレスを指す生ポインタも作れてしまう（危険！）
    let address = 0x012345usize;
    let _r = address as *const i32;
    // このポインタを参照外しすると未定義動作になる可能性がある
}


// ========================================
// 2. unsafeな関数やメソッドを呼ぶ
// ========================================

// unsafe fn として定義された関数は、unsafeブロック内でしか呼べない
// 「この関数のドキュメントを読んで、契約を守ることを約束しました」という宣言

unsafe fn dangerous() {
    println!("これはunsafeな関数です");
}

fn call_unsafe_fn() {
    // unsafe fn は unsafe ブロック内でしか呼べない
    unsafe {
        dangerous();
    }
    // 💡 unsafe関数の本体は丸ごとunsafeブロック扱いになるので、
    //    中で別のunsafe処理をするのに追加のunsafeブロックは不要
}


// ========================================
// 安全な抽象（Safe Abstraction）
// ========================================

// 関数がunsafeコードを含んでいても、関数全体をunsafeにする必要はない
// 中にunsafeブロックを閉じ込めて、外からは安全なAPIとして提供するのがベストプラクティス
// 標準ライブラリの多くもこの方法で実装されている

// 例: split_at_mut の自前実装
    // スライスを途中で2つに分割する関数
    // safeコードだけでは実装できない（同じスライスへの可変借用が2つ必要になるため）
    // でも人間には「前半と後半で重なっていない」とわかる → unsafeの出番

use std::slice;

fn split_at_mut(s: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = s.len();
    let ptr = s.as_mut_ptr();   // スライスの先頭を指す生ポインタ *mut i32 を取得

    assert!(mid <= len);        // 範囲外アクセスを防ぐガード

    unsafe {
        // slice::from_raw_parts_mut は生ポインタと長さからスライスを生成するunsafe関数
        (
            slice::from_raw_parts_mut(ptr, mid),                          // 先頭 → mid
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),           // mid → 末尾
        )
        // ptr.add(mid) でポインタを mid 分だけ進めた位置を取得
        // 💡 assertで mid <= len を保証しているので、ここのポインタは有効
    }
}

fn safe_abstraction_demo() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);
    // 外から見たら普通のsafe関数として呼べる
    println!("前半: {:?}", a);  // [1, 2, 3]
    println!("後半: {:?}", b);  // [4, 5, 6]
}


// ========================================
// extern関数（FFI: 外部関数インターフェイス）
// ========================================

// 他の言語（C言語など）で書かれた関数を呼び出す仕組み
// extern "C" { ... } ブロック内で関数シグネチャを宣言する
    // "C" はABI（Application Binary Interface）の指定
    // C言語のABIが最も一般的

// 書き方の例（実際にリンクするにはCのライブラリが必要なのでコメントで解説）:
//
// extern "C" {
//     fn abs(input: i32) -> i32;  // Cの標準ライブラリの abs 関数
// }
//
// fn ffi_demo() {
//     unsafe {
//         // 外部関数の呼び出しは常にunsafe（他言語の安全性をRustコンパイラが保証できないため）
//         println!("Cのabs(-3)の結果: {}", abs(-3));
//     }
// }

// 逆に、Rustの関数を他の言語から呼ばせることもできる
// #[no_mangle]  // コンパイラに関数名をマングル（改変）しないよう指示
// pub extern "C" fn call_from_c() {
//     println!("CからRustの関数が呼ばれました！");
// }
// 💡 この方向のexternにはunsafeは不要

fn ffi_demo() {
    println!("FFIの例: extern \"C\" {{ fn abs(input: i32) -> i32; }}");
    println!("→ Cの関数をRustから呼び出すにはunsafeブロック内で呼ぶ必要がある");
    println!("→ 逆にRustの関数をCから呼ばせるには #[no_mangle] pub extern \"C\" fn ... と書く");
}


// ========================================
// 3. 可変なstatic変数にアクセス・変更する
// ========================================

// Rustのグローバル変数 = static変数
    // 名前は SCREAMING_SNAKE_CASE（慣習）
    // 型注釈が必須
    // 'static ライフタイムの参照のみ格納可能

// 不変のstatic変数（アクセスはsafe）
static HELLO_WORLD: &str = "Hello, world!";

// 可変のstatic変数（アクセスも変更もunsafe）
static mut COUNTER: u32 = 0;

// 定数(const)とstatic変数の違い:
    // const  → 使われる度にデータが複製される可能性がある（インライン展開）
    // static → 固定されたメモリアドレスを持つ。常に同じ場所のデータにアクセスする
    //
    // 💡 可変なstatic変数が危険な理由:
    //   複数のスレッドが同時にアクセスするとデータ競合が起きるから
    //    可能なら16章で学んだ Mutex<T> や Arc<T> を使うべき
    //
    // ⚠️ Rust 2024 edition では static mut への直接参照が禁止された
    //    生ポインタ経由でアクセスするか、AtomicU32 等のスレッドセーフな型を使う

use std::ptr::{addr_of, addr_of_mut};

fn add_to_count(inc: u32) {
    unsafe {
        // addr_of_mut! で生ポインタを取得し、そこ経由で読み書きする
        let counter_ptr = addr_of_mut!(COUNTER);
        *counter_ptr += inc;
    }
}

fn static_variable_demo() {
    println!("挨拶: {}", HELLO_WORLD);  // 不変staticへのアクセスはsafe

    add_to_count(3);

    unsafe {
        let counter_ptr = addr_of!(COUNTER);
        println!("COUNTER: {}", *counter_ptr);  // 生ポインタ経由で読み取り
    }

    // 💡 現代的なRustでは static mut の代わりに AtomicU32 を使うのが推奨
    //    use std::sync::atomic::{AtomicU32, Ordering};
    //    static COUNTER: AtomicU32 = AtomicU32::new(0);
    //    COUNTER.fetch_add(3, Ordering::SeqCst);  // unsafeなしでスレッドセーフ

}


// ========================================
// 4. unsafeなトレイトを実装する
// ========================================

// トレイトのメソッドにコンパイラが確認できない不変条件（invariant）がある場合、
// そのトレイト自体を unsafe trait として宣言する
// 実装する側も unsafe impl で「不変条件を自分が保証する」と宣言する

unsafe trait Foo {
    fn name(&self) -> &str;
}

unsafe impl Foo for i32 {
    fn name(&self) -> &str {
        "i32です"
    }
}

// 実用例: Send と Sync トレイト
    // 型が全て Send + Sync な要素で構成されていれば、コンパイラが自動実装してくれる
    // 生ポインタなど非 Send/Sync な要素を含む型を Send/Sync にしたい場合は、
    // unsafe impl Send for MyType {} のように手動でマークする必要がある
    // → 「スレッド安全性は自分が保証します」という契約

fn unsafe_trait_demo() {
    let num: i32 = 42;
    println!("unsafeなトレイトFoo: {}", num.name());
}


fn main() {
    println!("=== 1. 生ポインタの参照外し ===");
    raw_pointers();

    println!("\n=== 2. unsafeな関数の呼び出し ===");
    call_unsafe_fn();

    println!("\n=== 安全な抽象 ===");
    safe_abstraction_demo();

    println!("\n=== extern関数（FFI）の概念 ===");
    ffi_demo();

    println!("\n=== 3. 可変なstatic変数 ===");
    static_variable_demo();

    println!("\n=== 4. unsafeなトレイト ===");
    unsafe_trait_demo();
}

