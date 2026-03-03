// Dropトレイト
    // 値がスコープを抜けるときに実行されるコードをカスタマイズする
    // スマートポインタ（Box<T>など）の実装に不可欠な機能

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // dropメソッドにクリーンアップ処理を書く
    // メモリの解放やファイルハンドルのクローズなどに使われる
    fn drop(&mut self) {
        println!("CustomSmartPointerのデータ `{}` をドロップします！", self.data);
    }
}

fn create_pointers() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointerを作りました。");
    // ここでスコープを抜けるため、d -> c の順（変数が作られた逆順）に変数が破棄され、dropが自動的に呼ばれる
}

//手動ドロップ
    //Dropトレイトとは独立したstd::mem::dropを使用する
fn manual_drop() {
    let c = CustomSmartPointer {
        data: String::from("データ"),
    };
    println!("CustomSmartPointerを作りました");

    drop(c); //preludeに含まれるので直接呼び出せる

    println!("CustomSmartPointerがmainの前でドロップしました")
}

fn main() {
    create_pointers();
    println!("\n===================");

    manual_drop();
}
