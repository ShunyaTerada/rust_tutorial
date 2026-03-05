fn main(){
    //エラーには二種類ある
        //回復不能なエラー：panic!マクロ
        //回復可能なエラー：Result<T, E>


    //panic!マクロ
        //失敗メッセージを出力し、スタックを巻き戻して片付け、プログラムを終了させる
    panic!("========\ncrash!\n========");    

    //バックトレースを遡ってエラーの過程を分析できる


    //Result型
    enum _Result<T, E> {
        Ok(T),
        Err(E),
    }
    
    //戻り値がResultな関数の使用
    use std::fs::File;
    let greeting_file_result = File::open("hello.txt"); //File::openはResult<T, E>を返す

    //返される三つの情報
        //成功したか失敗したか
        //成功値の型：std::fs::Fileで埋められたファイルハンドリングによるOkインスタンス
        //エラー値の型：std::io::ErrorによるErrインスタンス
    

    //match文の使用
    use std::io::ErrorKind; //io処理の結果発生する可能性のある色々な種類のエラーを表す
    let greeting_file = match greeting_file_result {
        Ok(file) => file, //Resultの列挙子は標準化されているのでResult::をしなくてもいい

        //内側にmatch式を追加し、エラーごとの対応を細分化
        Err(error) => match error.kind(){ //.kind：呼び出すとio::ErrorKind値が得られる
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("ファイルを作成するのに問題があります：{:?}", error),
            },
            other_error => {
                panic!("ファイルを開くのに問題がありました：{:?}", other_error);
            }
        },
    };


    //unwrap
    let bye_file = File::open("bye.txt").unwrap();
        //成功時：Okの中身を返す
        //失敗時：panic!マクロを呼ぶ

    //expect
    let expect_file = File::open("expect.txt")
    .expect("expect.txtが含まれるべきです");
        //失敗時のpanic!マクロのメッセージ内容を指定できる
    

    //エラーの委譲
    //発生個所で処理せず、呼び出し元がどうするか決められるようにエラーを返す
    use std::io::{self, Read};
    
    fn read_username_from_file() -> Result<String, io::Error> {
        //該当ファイルを開く
        let username_file_result = File::open("name.txt");

        //開こうとした結果のパターン
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),//panic!を呼び出す代わりに、File::openから得たエラー値を早期に返す
        };

        //書き込み先の文字列型を初期化
        let mut username = String::new();

        //ファイルの中身を読み込み、usernameに書き込もうとしたときのパターン
        match username_file.read_to_string(&mut username) { //読みだし内容を書き込むので可変参照にしている
            Ok(_) => Ok(username),
            Err(e) => Err(e),//最後の戻り値になるので明示的にreturnをつける必要はない
        }
    }
    //返したOk値やErr値をどう扱うかは関数の呼び出し元に任せる


    // ? 演算子
        //Result値を返す処理の末尾に?を置き、成功したらOk値を返してプログラムを継続、失敗したらErr値を返させる
        //match式との違い：型変換を行うfrom関数を通り、受け取ったエラー型をシグネチャの戻り値部分のエラー型に変換させる

    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username_file = File::open("name.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

        //メソッドチェーンによる省略
    fn read_username_from_file3() -> Result<String, io::Error> {
        let mut username = String::new();
        //処理を一度に済ませられるのでusername_file変数の中継がいらない
        File::open("hello.txt")?.read_to_string(&mut username)?;


        Ok(username)
    }

    //注意点として、?演算子を使用する際は関数シグネチャで戻り値型をResult（またはOption,FromResidual）にし、演算子の想定と互換性を持たせる必要がある
    //またResultやOptionの?演算子を同一関数内で混ぜて使用することはできない

        //なお、上記はよくある操作なので標準ライブラリfsで定義されている
    use std::fs;

    fn read_username_from_file4() -> Result<String, io::Error> {
        fs::read_to_string("name.txt")
    }


    //エラーハンドリング方法の判断基準（9-3 panic!すべきかするまいか）

        //基本方針
            //panic!を呼ぶと → 回復不可能という決定を、呼び出し側に代わって下すことになる
            //Resultを返すと → 呼び出し側に回復するかどうかの選択肢を与えることになる
            //→ 迷ったらResultを返すほうが良い第一選択肢


        //panic!が適切な場面

            //1. 例・プロトタイプコード・テスト
                //unwrapやexpectは「あとでちゃんとエラー処理を書くよ」というプレースホルダーとして使える
                //開発速度を優先したい段階ではunwrap/expectでマーカーを残しておく
                //テストコードでは失敗=テスト失敗にしたいので、unwrap/expectはまさに適切

            //2. コンパイラよりもプログラマがより情報を持っている場合
                //例：自分の誕生日や、ハードコードされた定数など
                //プログラマには絶対に正しいとわかっているが、コンパイラにはそれが判断できない
                //→ Resultが返される以上、形式上はErr分岐が必要だが
                //  「これは絶対にOkになる」と確信があるならexpectを使い、理由をメッセージに残す
    use std::net::IpAddr;
    let _home: IpAddr = "127.0.0.1"
        .parse()
        .expect("ハードコードされたIPアドレスは有効であるべき");
                //もしこれが失敗するなら、ハードコード自体に重大な欠陥があるということ → panic!で中断が妥当

            //3. 悪い状態（bad state）に陥ったとき
                //無効な値・矛盾する値・欠けた値がコードに渡された場合で、
                //かつ以下のいずれかに該当する場合：
                //  - その状態が予期されていない（ユーザ入力ミスとは異なる）
                //  - 以降のコードがこの悪い状態にないことに依存している
                //  - 使用している型にこの情報をコード化する手段がない

            //4. セキュリティに関わる危険な状態
                //不正な値での処理はコードを脆弱性に晒す可能性がある
                //例：境界外メモリアクセス → 標準ライブラリはpanic!する
                //  インジェクション攻撃のような入力 → 即座に中断すべき

            //5. 関数の「契約」が侵害されたとき
                //関数には契約（前提条件）がある：入力が特定の条件を満たすとき振る舞いが保証される
                //契約違反は常に呼び出し側のバグを示唆する
                //→ 呼び出し側に回復させるエラーではなく、バグ修正を促すためにpanic!が適切
                //→ APIドキュメントでpanic!の条件を明記すべき


        //Resultが適切な場面（panic!すべきでない場面）
            //失敗が予測できる場合はResultを返す
            //例：不正データを渡されたパーサ、レートリミットのHTTPレスポンスなど
            //→ 呼び出し側が処理方法を決定すべき「予測可能な失敗」


        //検証のために独自の型を作る
            //毎回if文でチェックするのは冗長 → 型に検証ロジックを閉じ込める
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                //契約違反 → panic!で呼び出し側のバグを通知
                panic!("予想の値は1から100の範囲でなければなりませんが、{}でした。", value);
            }
            Guess { value }
        }

        //ゲッター：valueフィールドが非公開なので、取得用メソッドが必要
        pub fn value(&self) -> i32 {
            self.value
        }
    }
            //valueを非公開にすることで、Guess::new経由でしかインスタンスを作れない
            //→ 範囲外の値がGuessに存在する手段がないことを型レベルで保証


        //型システムを活用したエラー防止
            //Rustの型システム自体が多くのチェックを行ってくれる
            //Option型 → 値がない可能性をコンパイル時に強制、SomeとNoneの処理を要求
            //u32型 → 負の数が来ないことをコンパイル時に保証
            //独自型（Guess等） → ビジネスロジック上の制約を型に閉じ込められる
            //→ 実行時チェックを減らし、コンパイル時の安全性を最大化できる


    //まとめ
        //panic!：プログラムが処理できない状態 → プロセスを中止
        //Result：回復可能な失敗 → 呼び出し側に処理を委ねる
        //適切な場面で使い分けることで、コードの信頼性が向上する
}