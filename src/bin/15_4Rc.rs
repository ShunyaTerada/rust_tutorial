// Rc<T>とは
    //参照カウント方式のスマートポインタ
    // Reference Counting

    // なぜ必要か？
        // グラフデータなどでノードは複数のエッジから所有されている状態になる
        // そのため、参照の数が0にならない限りドロップされるべきではない
        // なので、参照の数を管理する必要がある
        // しかし、通常の所有権では最初のエッジが作成された時点で所有権がムーブされるので、他のエッジは作れない
        // 
    // Rc<T>はシングルスレッドを想定している
        // マルチスレッドでは別のアプローチを採用する


// データの共有
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

    // 所有権を奪うのではなく、aが保持しているRc<List>をクローンする
        // 通常のクローンとは異なり、ディープコピーはせず、Rc<List>内のデータの参照カウントを増やすだけ
        // 参照がドロップするたびにRc<T>のDropトレイトの実装により、カウントが減る
fn data_sharing() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a作成後のカウント：`{}", Rc::strong_count(&a)); // 参照カウントの出力はstrong_count(), 別用途でweak_countもあるので注意


    let b = Cons(3, Rc::clone(&a)); // 通常のcloneと区別するためにメソッド記法ではなくパス形式にする
    println!("b作成後のカウント：`{}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("c作成後のカウント：`{}", Rc::strong_count(&a));
    }
    println!("cがスコープを抜けた後のカウント：{}", Rc::strong_count(&a))
}
fn main() {
    data_sharing();
}
