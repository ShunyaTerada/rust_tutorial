fn main(){
    borrow_reference();
    println!("==========\n");

    mutable_reference();
    println!("==========\n");

    deffelence_scope();
    println!("==========\n");

    com_reference();
    println!("==========\n");
}

//参照の借用
fn borrow_reference(){
    let s1 = String::from("hello");

    let len = calculate_length(&s1); //&で参照の借用を表す

    println!("{}の長さは{}です", s1, len);
}

 //引数の型でも&で借用を表す
fn calculate_length(s: &String) -> usize { 
    s.len()
}//sはスコープ外になるが、借用中の参照しているものの所有権を持っているわけではないので、ドロップはされない

//可変参照
fn mutable_reference(){
    let mut s = String::from("hello");//借用対象を可変にしておく

    change(&mut s);//&の後にmutをつけることで可変参照を借用できる

    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", borrow");
}

//注意点：ある値への可変参照が存在するなら、その値への参照を他に作ることはできない。

//別スコープなら新しい参照を借用できる
fn deffelence_scope (){
    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    }//ここでr1がスコープを抜けてドロップする

    let r2 = &mut s;
    println!("{r2}");
}

//可変と不変の参照の組み合わせもエラーになる
fn com_reference(){
    let mut s = String::from("helo");

    let _r1 = &s;
    let _r2 = &s;//不変参照どうしなら問題ない
    
    let _r3 = & mut s;//可変が組み合わさるとダメ

    //r3の前にprintln!("{} and {}", r1, r2);などでsのスコープを終わらせていれば問題ない
}

//参照は常に有効でなければならない
fn _dangle(){ // dangleはStringへの参照を返す

    let _s = String::from("hello"); // sは新しいString

    // &s  // String sへの参照を返す
} // ここで、sはスコープを抜け、ドロップされる。そのメモリは消されてむこうな参照が残る





