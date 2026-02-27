// クロージャとは？
    // 変数に保存したり、引数として渡せる匿名関数
    // 関数と違い、定義されたスコープの値をキャプチャできる
    //ちなみにＪＳの名前付き関数とアロー関数はどちらもクロージャ

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        //Someだったらその内容を、Noneだったらクロージャー内の関数を呼び出す
        user_preference.unwrap_or_else(|| self.most_stocked()) //giveawayの&selfをキャプチャしている
        // 文脈が限定されているので、型注釈しなくても推論してくれる
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for  color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        }else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "好み{:?}を持つユーザーは{:?}を得ます",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "好み{:?}を持つユーザーは{:?}を得ます",
        user_pref2, giveaway2
    );
}

fn closure_immutable() {
    let mut list = vec![1, 2, 3];

    println!("Before defining closure: {:?}", list);

    let mut borrows_immutably = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
  
    borrows_immutably();

    println!("After calling closure: {:?}", list);
}

fn closure_mutable() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //可変参照が有効な間は他の参照はできない

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

//所有権を奪いたいときはmoveキーワードを先頭に置く  
use std::thread;
fn closure_moving_ownership() {
    let list = vec![1, 2, 3];
    println!("クロージャ定義前: {:?}", list);

    thread::spawn(move || println!("スレッドから：{:?}", list))
    .join()
    .unwrap();
}


// クロージャがどのように環境をキャプチャするかによって以下のトレイトのいずれかもしくは全てが自動的に付加される
    // FnOnce: 全てのクロージャに付加。
        // 一度だけ呼び出せる
        // キャプチャした値をムーブする場合はこれだけが付加される
    // FnMut: ムーブはしないが可変参照するかもしれない場合に付加。
        // 複数回呼び出せる
    // Fn: 値をムーブも変更もしない、もしくはキャプチャしない場合に付加
        // 並行して複数回呼び出せる
