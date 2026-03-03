// 安全な並行性を保証するには？
// スレッドやアクターがメッセージを相互に送りあう
    // メモリを共有しない
// Rustではデータ送信の手段として、チャンネルを実装している
    // チャンネルは、転送機と受信機に分けられる
    // どちらかがドロップされるとチャンネルが閉じられたという


// チャンネルの生成
use std::sync::mpsc;
use std::thread;
// multiple producer, single consumer
    // 複数の生産者と一つの消費者

fn channel() {
// メインスレッド

    let (tx, rx) = mpsc::channel(); // 2つの要素をもつタプルを返す
    // 1つ目の要素（tx）: 転送側
    // 2つ目の要素（rx）: 受信側

// サブスレッド
    thread::spawn(move || { 
        let val = String::from("ハァイ！！");

        // 転送側のsendメソッドでデータを送る
            // sendメソッドはResult<T, E>を返す
            // エラーの時にパニックするためにunwrapを呼び出している
        tx.send(val).unwrap(); 
        // この時、valの所有権が受信側にムーブされるので、サブスレッドでは無効になる
    }); // サブスレッドここまで

    // 受信側のrecvメソッドでデータを受け取る
        // メインスレッドの実行をブロックし、データを受信するまで待機する
        // 一旦値が送信されたら、recvはそれをResult<T, E>に含んで返す 
        // 転送機が閉じたら、recvはエラーを返し、もう値は来ないと通知する
    let received = rx.recv().unwrap();
    println!("取得データ: {}", received);

    // try_recvメソッド
        // メインスレッドをブロックせず、即座にResult<T, E>を返す
            // メッセージがあったら、それを含むOk値、
            // 何もメッセージがなければ、Err値
        // メッセージがあったら処理し、それ以外は他の作業ができる
}

// 複数のデータを送信する場合
use std::time::Duration;
fn multi_data() {
    let (tx, rx) = mpsc::channel();
    // ちなみに、txをクローンして、複数スレッドからデータ転送することもできる

    thread::spawn(move || {
        let vals = vec![
            String::from("ハァイ"),
            String::from("ハァイ!"),
            String::from("ハァイ!!"),
            String::from("ハァイ!!!"),
        ];

        for val in vals { 
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rx(受信機)はIteratorトレイトを実装しているため、forループで回せる
    // ループが回るたびに裏側で暗黙的に rx.recv() が呼ばれている
    // 転送側(tx)がすべてドロップされて通信が閉じると、Noneを返して安全にループを抜ける
    for received in rx { 
        println!("取得データ: {}", received);
    }
}


fn main() {
    channel();
    multi_data();
}