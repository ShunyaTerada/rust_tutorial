// 現代のコンピューターは複数のプロセスを管理しいる
// プロセス内で独立して走らせる機能をスレッドと呼ぶ
// 複数のスレッドにより、効率的な処理が可能だが、バグにもつながりやすい
// Rustは、この並行性に起因するバグを所有権と型システムによる解決を試みている


// spawnで新規スレッド生成
use std::thread;
use std::time::Duration;

fn thread_spawn() {
    //サブスレッド生成
    let handle = thread::spawn(|| { // 戻り値：JoinHandle<()>
        for i in 1..10{
            println!("新規スレッド: {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("メインスレッド: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // JoinHandle<()>は、そのjoinメソッドを呼び出したとき、スレッドの終了を待つ所有された値

    // 立上げたスレッドがメインが終了するまえに確実に終了させる
        // メインスレッドはブロックされる
    handle.join().unwrap();
}


// スレッドでmoveクロージャを使用する
    // メインスレッドからデータをキャプチャする
fn move_closure() {
    let v = vec![1, 2, 3];

    // 借用だとライフタイムの不整合が起こる可能性があるため、ムーブした方が合理的
    let handle = thread::spawn(move || {
        println!("ベクタ: {:?}", v);
    });

    handle.join().unwrap();
}


fn main() {
    thread_spawn();
}