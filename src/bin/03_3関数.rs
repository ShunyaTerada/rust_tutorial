fn main(){
    hello_function();//呼び出しもとから捉えられるスコープ内で定義されていれば、順番は気にしない
    
    with_argument(5, 23);

    statement_and_expression();

    let x = five();
    println!("five:{x}");

    return_statement();
}


fn hello_function(){
    println!("Hello Function!");
}

fn with_argument(x:i32, y: u32){ //必ず仮引数の型を指定する
    println!("argument:{x},{y}");
}

fn statement_and_expression(){
    let y = { //波括弧によるスコープは式にあたるので文である宣言の対象にできる
        let x = 3;
        x + 1 //式の後ろに；はつけない
    };

    println!("y:{y}");
}

fn five() -> i32 {// -> の後に戻り値の型を指定できる
    5
}

fn return_statement() -> () { //式が返されない場合の戻り値の型はユニットになる
    println!("It's statement");
}