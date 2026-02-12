//１．各値には、所有者が存在する。
//２．いかなる時も、所有者は一つである。
//３．所有者がスコープから外れたら、値は破棄される。

fn main() {
    //スタック領域のスコープ
    {   
        //sはまだ有効ではない
        let s = "hello";//ここからsが有効になる
        println!("{s}");
    }//スコープが終わり、sは無効になる

    println!("================\n");

    //以下、ヒープ領域で所有権を見ていく
    let mut s = String::from("hello");

    s.push_str(", heap");//リテラルをStringに付け加える

    println!("{}", s);

    //ムーブ
    let s1 = String::from("hello");
    let s2 = s1;//ここでデータがs2にムーブされ、s1が無効になる。

    println!("{}world", s2);

    ownership_with_fn();

    ownership_with_return();
}

fn ownership_with_fn() {
    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても大丈夫

} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

fn ownership_with_return (){
    let _s1 = gives_ownership(); //gives_ownershipは、戻り値をs1にムーブする

    let s2 = String::from("hello");//s2がスコープに入る

    let _s3 = takes_and_gives_back(s2);//s2はtakes_and_gives_bacにムーブされ、戻り値もs3にムーブされる
}//s1,s3がスコープを抜け、ドロップされる。s2は既にムーブされているので何も起きない

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string //呼び出し元関数にムーブされる
}

fn takes_and_gives_back(a_string: String) -> String { 
    a_string
}

