fn main(){
    println!("====================");

    println!("変数宣言\n");
    let mut x = 5; //再代入したければ mut が必要
    println!("xの値：{x}");
    x = 6;
    println!("xの値：{x}");

    println!("\n=================");

    println!("定数宣言\n");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS:{THREE_HOURS_IN_SECONDS}");
    // I/Oなどのすぐに値がわからない物は定義できない
    // 定数の命名は大文字にアンダースコアをつけるのが規則

    println!("\n=================");

    println!("シャドーイング\n");
    let x = 5;

    let x = x + 1;//代入はできないが再宣言はできるらしい・・・
    println!("xの値（一回目）：{x}\n");

    {
        let x = x * 2;
        println!("スコープ内でのXの値:{x}\n");
    }

    println!("xの値（二回目）：{x}\n");

    // ちなみに以下のように代入の場合、スコープ内で代入した値はスコープが終っても有効になる
    //二回目のxの値も12になる 

    // let mut x = 5;

    // x = x + 1;//代入はできないが再宣言はできるらしい・・・
    // println!("xの値（一回目）：{x}\n");

    // {
    //     x = x * 2;
    //     println!("スコープ内でのXの値:{x}\n");
    // }

    // println!("xの値（二回目）：{x}");

    // 型も変えられる
    let spaces = "  ";//文字列型
    let spaces = spaces.len();//数値型

    println!("spacesの値：{spaces}");

    println!("\n=================");

    println!("\n");
    
}