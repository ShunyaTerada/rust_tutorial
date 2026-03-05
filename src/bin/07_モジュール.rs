fn main(){
    eat_at_restaurant();
    println!("\n=======================");
    eat_at_res2();
    println!("\n=======================");
    eat_at_res3();
    println!("\n=======================");
    eat_at_res4();
    println!("\n=======================");
    eat_at_res4_difscope();
    println!("\n=======================");
    renamed_use();
    println!("\n=======================");
}

//モジュール
mod front_of_house { //modでモジュール定義
   pub mod hosting {   //pubで公開
        pub fn add_to_waitlist(){} //モジュールだけではなく、実際に使う要素も公開する必要がある

        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}

        fn serve_order(){}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant(){ 
    //絶対パス
    crate::front_of_house::hosting::add_to_waitlist();//兄弟モジュールは公開しなくても参照できる。ただし、その中の要素は要公開

    //相対パス
    front_of_house::hosting::add_to_waitlist();
    //親モジュールからはじめるときはsuper::～のようにはじめる
}


//構造体の公開
mod back_of_house {
    pub struct Breakfast {//モジュールと同じくpubで公開
        pub toast: String, //個別のフィールドも公開するにはpubをつける必要がある
        seasonal_fruit: String,
    }

    impl Breakfast { //構造体に非公開のフィールドがあるので公開されているインスタンス関数が必要
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_res2(){
    //夏にライムギパンの朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");

    //やっぱり別のパンにする
    meal.toast = String::from("Wheat");
    println!("{}トーストが欲しいです", meal.toast);

    //フルーツのフィールドは公開してないので指定できない
}


//enumの公開
mod back_of_h2 {
    pub enum Appetizer { //モジュールや構造体と異なり、内部の要素もすべて公開される
        Soup,
        Salad,
    }
}

pub fn eat_at_res3() {
    let order1 = back_of_h2::Appetizer::Soup;
    let order2 = back_of_h2::Appetizer::Salad;
}


//useキーワード
use crate::{customer::eat_at_res4_difscope, front_of_house::hosting}; //useで一度パスを指定する

pub fn eat_at_res4() {
    hosting::add_to_waitlist() //指定されたモジュールなどをパスを省略して使用できる
}

    //ただし、useと呼び出し側は同じスコープになければならない
mod customer {
    pub fn eat_at_res4_difscope() {//customerモジュールの中のため、useの有効スコープより一つ下層になる
        // hosting::add_to_waitlist(); 有効にするとコンパイルエラーになる
    }
}


//useパスの慣例：関数のパス指定はフルパスではなく、親までの指定に留める

    //悪い例
use crate::front_of_house::hosting::add_to_waitlist; 
pub fn eat_at_res5_bad(){
    add_to_waitlist; //関数がこのスコープで定義されたか他スコープで定義されたかわかりにくい
}

    //良い例
// use crate::front_of_house::hosting;  上記で指定済なのでここではコメントアウト
pub fn eat_at_res5_good() {
    hosting::add_to_waitlist();//どこで定義されたかわかりやすい
}

    //構造体やenumその他の要素はフルパスで指定することが多い（はっきりとした理由はわからないらしい）


//asで改名
use crate::front_of_house::hosting as fr_hos ;

fn renamed_use(){
    fr_hos::add_to_waitlist(); //改名後の名前を使用
}


//pub useで再公開
mod public_mod {
    pub mod anymod {
        pub fn any_function(){}
    }
}
pub use crate::public_mod::anymod;

pub fn use_pubfn(){
    anymod::any_function();
}


//外部パッケージの使用
    //乱数パッケージを使用する場合、Cargo.tomlにてrand = "0.8.5"を加えて導入してから以下のように使用できる
use rand::Rng;

fn use_package(){
    let secret_number = rand::rng().random_range(1..=100);
}


//共通の親を持つパスをまとめて指定する

    //通常の書き方
use std::cmp::Ordering;
use std::arch;

    //省略記法
use std::{cmp::Ord, any}; //std::という共通親の子パスを{}内にまとめて書いている

    //片方がもう一方のサブパスである場合
use std::io::{self, Write}; //selfでstd::io自身も指定することを示している

    //全ての公開要素を指定する(glob演算子)
use std::collections::*; // * で以降のすべての公開要素を指定している


//モジュール内の要素を別ファイルに分割
mod split_mod {
    mod component{ //この要素を分割したい
        enum com{
            a,
            b,
        }
    } 
}
    //同じディレクトリにcomponent.rsファイルを作成し
    //その中にenum comを書く
    //このファイルのmod componentの{}を削除する
    