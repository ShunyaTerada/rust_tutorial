// 高度なトレイト
// 公式: https://doc.rust-jp.rs/book-ja/ch19-03-advanced-traits.html

// 10章で学んだトレイトの応用編
// より複雑な型の関係性やパターンを表現するためのテクニックが登場する


// ========================================
// 1. 関連型（Associated Types）
// ========================================

// トレイト定義の中に「型のプレースホルダー」を置く仕組み
// 実装する側が、そのプレースホルダーに具体的な型を入れる

// 💡 既に使っていた！ Iterator トレイトの Item がまさに関連型
//    pub trait Iterator {
//        type Item;                        ← これが関連型
//        fn next(&mut self) -> Option<Self::Item>;
//    }

// では、なぜジェネリクス<T>ではなく関連型を使うのか？
    // ジェネリクスだと: impl Iterator<u32> for Counter, impl Iterator<String> for Counter ...
    //   → 1つの型に対して複数の実装ができてしまう
    //   → next() を呼ぶたびにどの実装か注釈が必要になる
    //
    // 関連型だと: impl Iterator for Counter { type Item = u32; } のように1つだけ
    //   → 「CounterのイテレータはItemがu32」と一意に決まる
    //   → 注釈なしで next() を呼べる
    //
    // 💡 「この型に対してこのトレイトの実装は1つだけ」にしたいときに関連型を使う

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;    // 関連型: このイテレータが返す値の型を u32 に確定

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn associated_types_demo() {
    let counter = Counter::new();
    // next() が返す型は自動的に Option<u32> に決まる
    for val in counter {
        println!("カウント: {}", val);
    }
}


// ========================================
// 2. デフォルトのジェネリック型引数と演算子オーバーロード
// ========================================

// ジェネリックな型引数に「既定の型」を指定できる
// 記法: <PlaceholderType=ConcreteType>

// 一番よく使われる場面: 演算子オーバーロード
    // Rustでは独自の演算子は作れないが、std::ops の演算子トレイトを実装することで
    // +, -, * などの振る舞いをカスタマイズできる

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Point + Point を可能にする
impl Add for Point {
    type Output = Point;    // 関連型: 加算結果の型

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 💡 Add トレイトの定義を見てみると:
//    trait Add<Rhs=Self> {     ← Rhs（Right Hand Side: 右辺）のデフォルトが Self
//        type Output;
//        fn add(self, rhs: Rhs) -> Self::Output;
//    }
//    impl Add for Point は impl Add<Point> for Point と同じ意味
//    デフォルト型引数のおかげで <Point> を省略できている

// 異なる型同士の加算も可能（デフォルトを上書き）
#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn operator_overloading_demo() {
    // 同じ型同士
    let p = Point { x: 1, y: 0 } + Point { x: 2, y: 3 };
    println!("Point + Point = {:?}", p);

    // 異なる型同士
    let total = Millimeters(500) + Meters(1);
    println!("500mm + 1m = {:?}", total);
}


// ========================================
// 3. 明確化のためのフルパス記法（完全修飾構文）
// ========================================

// 同じ名前のメソッドが複数のトレイト・型自体に存在するとき、
// どれを呼ぶか明示する必要がある

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("機長のお言葉: ご搭乗ありがとうございます");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("上がれ！（魔法使い）");
    }
}

impl Human {
    fn fly(&self) {
        println!("*激しく腕を振る*（人間）");
    }
}

fn disambiguation_demo() {
    let person = Human;

    // 型に直接実装されたメソッドがデフォルトで呼ばれる
    person.fly();                   // *激しく腕を振る*

    // 特定のトレイトのメソッドを呼びたいときはトレイト名::メソッド(&インスタンス) で指定
    Pilot::fly(&person);            // 機長のお言葉
    Wizard::fly(&person);           // 上がれ！
}

// 💡 self引数を持たないトレイトメソッド（関連関数）の場合は、
//    フルパス記法（完全修飾構文）が必要:
//    <Type as Trait>::function_name()
//
//    例: <Human as Pilot>::name()
//    → 「HumanをPilotとして見たときの name() を呼ぶ」

trait Animal {
    fn baby_name() -> String;      // selfがない → 関連関数
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("スポット")     // 犬の固有名
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("子犬")         // 動物としての一般名
    }
}

fn fully_qualified_syntax_demo() {
    println!("Dog::baby_name() = {}", Dog::baby_name());  // スポット

    // Animal::baby_name() だけだと、Rustはどの型のAnimal実装か分からないのでエラー
    // フルパス記法で「DogのAnimal実装」を明示する
    println!("<Dog as Animal>::baby_name() = {}", <Dog as Animal>::baby_name());  // 子犬
}


// ========================================
// 4. スーパートレイト
// ========================================

// あるトレイトが、別のトレイトの機能を前提条件にする仕組み
// 💡 「このトレイトを実装するには、先にあのトレイトも実装しておいてね」という制約

// 例: OutlinePrint は fmt::Display を前提にしている
//     → to_string() を使いたいから
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();  // Display があるから to_string() が使える
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// OutlinePrint を Point に実装するには、まず Display を実装する必要がある
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}  // Display が実装済みなので OK

fn supertrait_demo() {
    let p = Point { x: 1, y: 3 };
    p.outline_print();
    // 出力:
    // **********
    // *        *
    // * (1, 3) *
    // *        *
    // **********
}


// ========================================
// 5. ニュータイプパターン（外部の型に外部のトレイトを実装する）
// ========================================

// 孤児ルール（Orphan Rule）の復習:
    // トレイトか型のどちらかが自分のクレートに属していないと、そのトレイトをその型に実装できない
    // 例: Vec<T> に Display を実装したい → 両方とも標準ライブラリ所属なので直接はNG

// ニュータイプパターンで回避する:
    // タプル構造体で既存の型を薄くラップして、「自分のクレートの型」にしてしまう
    // ラッパ型はコンパイル時に消えるので実行時コストはゼロ！

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0 で中身の Vec<String> にアクセス（タプル構造体の0番目）
        write!(f, "[{}]", self.0.join(", "))
    }
}

// 💡 欠点: Wrapper は Vec<String> のメソッドを持っていない
//    解決策1: Deref トレイトを実装して内部の型を透過的に使う
//    解決策2: 必要なメソッドだけを手動で委譲する

fn newtype_pattern_demo() {
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
    ]);
    println!("Wrapperの表示: {}", w);  // [hello, world]
}


fn main() {
    println!("=== 1. 関連型 ===");
    associated_types_demo();

    println!("\n=== 2. 演算子オーバーロード ===");
    operator_overloading_demo();

    println!("\n=== 3. フルパス記法（完全修飾構文） ===");
    disambiguation_demo();
    fully_qualified_syntax_demo();

    println!("\n=== 4. スーパートレイト ===");
    supertrait_demo();

    println!("\n=== 5. ニュータイプパターン ===");
    newtype_pattern_demo();
}
