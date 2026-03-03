// 循環参照とメモリリークの回避（Weak<T>）
    // Rc<T>を使用すると、お互いを参照し合う「循環参照」を作れてしまう。
    // 循環参照が起きると、参照カウント(strong_count)が永遠に0にならず、メモリリークが発生する。
    
    // 解決策としての Weak<T> (弱い参照)
    // - Rc::clone は strong_count を増やす（これが0にならないとドロップされない）。
    // - Rc::downgrade を使うと、Weak<T> という「弱い参照」を作れる。これは weak_count を増やす。
    // - weak_count がいくつあっても、strong_count が0になれば値はドロップされる。
    // - Weak<T> の値にアクセスするには upgrade() メソッドを使う。値がドロップ済みかもしれないので Option<Rc<T>> が返る。

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 木構造（ツリー）のノード
// 子ノードは複数持てるように Vec<Rc<Node>> を使う
// 親ノードは循環参照を防ぐために Weak<Node> を使う（子は親を所有しない）
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // 1. leaf(葉)ノードを作成
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // 最初は親を持たない
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    // leafの親を確認。upgrade()はOptionを返す。まだ親がいないので None になるはず
    println!("leafの親 = {:?}", leaf.parent.borrow().upgrade());

    {
        // 2. branch(枝)ノードを作成。leafを子として持つ
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]), // leafのstrong_countが増える
        });

        // leafの親に、branchの弱い参照(Weak<Node>)をセットする
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        // branch がスコープを抜けるのでドロップされる。
        // branch の strong_count は 0 になり、メモリ解放される。
        // その時、子である leaf の strong_count も 1 減る。
    }

    // 3. branchドロップ後のleafの状態を確認
    // branchは既にドロップされているので、leaf.parent.upgrade() は None を返す（親が削除されても安全に扱える）
    println!("leafの親 = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}