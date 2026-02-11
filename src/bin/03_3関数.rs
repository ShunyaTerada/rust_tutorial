fn main(){
    hello_function();//呼び出しもとから捉えられるスコープ内で定義されていれば、順番は気にしない
    with_argument(5);
}

fn hello_function(){
    println!("Hello Function!");
}

fn with_argument(x:i32){
    println!("argument:{x}");
}