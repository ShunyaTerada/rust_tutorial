// Rustのオブジェクト指向プログラミング（OOP）機能
// 公式: https://doc.rust-jp.rs/book-ja/ch17-00-oop.html


// RustはOOP言語か？
    // OOPの特徴である「カプセル化」はRustにもある（pubによる公開範囲の制御など）
    // しかし「継承」は存在しない
        // 代わりに「トレイト」でコードの共有やポリモーフィズムを実現する
        // 継承より柔軟で、不要なメソッドまで引き継ぐ問題（ダイヤモンド問題等）も起きない


// トレイトオブジェクト（異なる型の値を同じように扱う）

// なぜ必要か？
    // 例えばGUI画面を描画するとき、ボタン(Button)やテキストボックス(TextField)など
    // 「全く異なるデータ構造」だけど「全て draw() できる」要素を
    // 1つのVecにまとめてforループで一気に処理したいことがある
    //
    // ジェネリクス(<T>)だと、コンパイル時に型が確定するので
    // 「全部Button」か「全部TextField」のどちらか1種類しか入れられない（静的ディスパッチ）
    //
    // 💡 複数の異なる型を同じVecに同居させたいときに使うのが「トレイトオブジェクト」！

pub trait Draw {
    fn draw(&self);
}

// Button構造体とTextField構造体（それぞれ独自のフィールドを持つ）
pub struct Button { pub width: u32, pub label: String }
impl Draw for Button { fn draw(&self) { /* ボタンを描画 */ } }

pub struct TextField { pub is_focused: bool, pub text: String }
impl Draw for TextField { fn draw(&self) { /* テキストフィールドを描画 */ } }

// 画面（Screen）は、異なるUI要素を1つのリストで保持したい
pub struct Screen {
    // これがトレイトオブジェクトの配備！
    // Drawトレイトを実装していれば、どんな型でもこのVecに入れられる
    // 「dyn」は dynamic（動的）の略
        // 実行時まで中身の型がわからないため、サイズが不定になる
        // なので必ず Box<T> や & などのポインタ経由で扱う（サイズを固定するため）
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw(); // 実行時にどの型の draw が呼ばれるか動的に決まる（動的ディスパッチ）
        }
    }
}


// 状態パターンとRustらしいアプローチ

// 古典的なOOPのやり方
    // ブログ記事（Draft → PendingReview → Published）のように状態が変わるものを実装する際、
    // 「状態オブジェクト」を内部に持たせて動的に切り替える手法を使う

// 💡 Rustらしいやり方（Type-Driven Design）
    // 「型の変換」によって状態を表現する
    // 例: Post::new() で DraftPost型 を返し、request_review(self) を呼ぶと PendingReviewPost型 を返す
    // コンパイル時点で「Draft状態のまま Publish しようとする」といったロジックミスを
    // 型システムが完全に防いでくれる！
    //
    // ※ 古典的な状態パターンだと、不正な状態遷移は実行時エラーになるが、
    //    Rustでは「そもそもコンパイルが通らない」ので安全


// 💡 Rustの定石: Option::take() による所有権の奪取
    // 構造体のフィールド（&mut self でしかアクセスできないもの）から、
    // 値の「所有権」を完全に奪って別のメソッド（selfを要求するもの）に渡したい場合によく使うハック。
    // 例: 17章の状態パターン実装や、20章のスレッドの join(self) 時など。
    // 
    // Rustでは構造体の一部だけをムーブして未初期化状態にすることは許されない。
    // そのため、フィールドを Option<T> で包んでおき、.take() メソッドを呼ぶ。
    // take は、中の Some(T) を取り出して所有権ごと返し、元のフィールドには安全な None を残す。
    //
    // 使い方（if let との組み合わせ）:
    // if let Some(val) = self.state.take() { 
    //     // valの所有権を奪った！元の self.state は None になっている。
    //     // ここで val を使って自由に処理（joinなど）ができる。
    // }


fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button { width: 50, label: String::from("OK") }),
            Box::new(TextField { is_focused: false, text: String::from("Rust") }),
        ],
    };

    screen.run();
    println!("17章のOOP機能の概念まとめました。実行時のポリモーフィズムはBox<dyn Trait>を使います。");
}
