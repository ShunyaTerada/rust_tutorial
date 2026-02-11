fn main() {

    compare_five(3);
    println!("=================\n");

    judge_10(10);
    println!("=================\n");


    //定値関数を配列に格納してforでループ実行させる
    let fn_array = 
    [
    loop_ten_times,
    loop_with_label,
    use_while,
    for_with_range,
    ];

    for now_fn in fn_array {
        now_fn();
        println!("=================\n");
    }
}

fn compare_five(number:i32) -> (){
    //条件式はbool型にする必要がある
    //各アームの戻り値の型が一致する必要がある
    if number < 5 {
        println!("condition was bigger more 5");
    }else if number == 5{
        println!("condition was 5");
    }else {
        println!("condition was smaller less 5");
    }
}

fn judge_10(number:i32) -> (){
    //Rustのifは式なので変数にバインドできる
    let result = if number == 10 { "１０です" } else { "１０じゃないです" };
    println!("この値は{result}");
}

fn loop_ten_times(){
    let mut acc = 0;
    loop {
        acc += 1;
        println!("{acc}回目");
        //breakの後に戻り値を指定できる。
        if acc >= 10 {break println!("終了！");}
    }
}

fn loop_with_label(){
    let mut count = 0;
    //ループラベルはシングルクォートで始める
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 8 {
                break;
            }
            if count == 3 {
                //ループラベルを指定すれば内ループから外ループを終了できる
                break 'counting_up;
            }
            remaining -= 1;
        }

        count+= 1;
    }
    println!("カウント：{count}");
}

fn use_while(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("終了")
}

fn for_with_range() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}