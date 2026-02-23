// 12章 入出力プロジェクト：コマンドラインプログラム（minigrep）を構築する
// ※ 実際のプロジェクトは別ディレクトリ(minigrep)で進行。ここは概念まとめ。

fn main() {

    // ========================================
    // 12-1: コマンドライン引数を受け付ける
    // ========================================

    //std::env::args()でコマンドライン引数のイテレータを取得
    use std::env;
    let args: Vec<String> = env::args().collect(); //collectでベクタに変換
        //args[0] はプログラム名
        //args[1] 以降がユーザーの引数

    //引数を変数に保存する
    //let query = &args[1];      //検索文字列
    //let file_path = &args[2];  //検索対象ファイル

    //実行例: cargo run -- 検索文字列 poem.txt


    // ========================================
    // 12-2: ファイルを読み込む
    // ========================================

    use std::fs;
    //fs::read_to_string(ファイルパス) でファイル内容をStringとして読み込む
    //戻り値はResult<String, io::Error>
    let _contents = fs::read_to_string("poem.txt")
        .expect("ファイルを読み込むことができるはずでした");


    // ========================================
    // 12-3: リファクタリングでモジュール性とエラー処理を向上させる
    // ========================================

    //問題点：main関数が複数の責任を持っている → 分割する
    //ガイドライン（バイナリプロジェクトの関心の分離）：
        //1. main.rs: 引数の解析、設定の構築、run関数の呼び出し、エラー処理
        //2. lib.rs:  プログラムのロジック全般

    //設定を構造体にまとめる
    struct Config {
        query: String,
        file_path: String,
    }

    //コンストラクタパターン（newではなくbuildを使う慣習）
    //→ 失敗する可能性があるので、Result型を返す
    impl Config {
        fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("引数が足りません");
            }
            let query = args[1].clone();     //.clone()で所有権を持つコピーを作る
            let file_path = args[2].clone(); //効率は落ちるが、ライフタイム管理がシンプルに
            Ok(Config { query, file_path })
        }
    }

    //main関数でのエラー処理：unwrap_or_elseとprocess::exit
    use std::process;

    let config = Config::build(&args).unwrap_or_else(|err| {
        //引数の解析に問題がありました
        println!("Problem parsing arguments: {}", err);
        process::exit(1); //異常終了
    });

    //ロジックをrun関数に抽出（本来はlib.rsに置く）
    use std::error::Error;

    fn run(config: &Config) -> Result<(), Box<dyn Error>> {
        //Box<dyn Error>: トレイトオブジェクト。あらゆるエラー型を返せる
        let _contents = fs::read_to_string(&config.file_path)?; //?演算子でエラーを委譲
        Ok(())
    }

    //run関数の呼び出しとエラー処理
    if let Err(e) = run(&config) {
        println!("アプリケーションエラー: {}", e);
        process::exit(1);
    }


    // ========================================
    // 12-4: テスト駆動開発（TDD）でライブラリの機能を開発する
    // ========================================

    //TDDのプロセス：
        //1. 失敗するテストを書き、想定通りの理由で失敗することを確認
        //2. テストを通過させるのに十分なコードを書く
        //3. リファクタリングしてテストが通り続けることを確認
        //4. 繰り返す

    //search関数の実装
    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        //ライフタイム注釈：戻り値の参照はcontentsと同じ寿命を持つ
        let mut results = Vec::new();
        for line in contents.lines() { //.lines()で各行のイテレータを取得
            if line.contains(query) {   //.contains()で文字列の検索
                results.push(line);
            }
        }
        results
    }

    let text = "Rust: safe, fast, productive.\nPick three.\nDuct tape.";
    let result = search("duct", text);
    println!("検索結果: {:?}", result); // ["safe, fast, productive."]


    // ========================================
    // 12-5: 環境変数を取り扱う
    // ========================================

    //大文字小文字を無視する検索を環境変数で制御する

    fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase(); //to_lowercase: 小文字化（新しいStringを返す）
        let mut results = Vec::new();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) { //&queryにするのはcontainsが&strを期待するため
                results.push(line);
            }
        }
        results
    }

    //環境変数の読み取り
    //env::var("変数名") → Result<String, VarError>
    //  Ok(値)  : 変数がセットされている
    //  Err(_)  : 変数がセットされていない
    let ignore_case = env::var("IGNORE_CASE").is_ok(); //.is_ok()でboolに変換

    //設定に組み込む例：
    //  Configにignore_case: boolフィールドを追加し、
    //  run関数内でsearchとsearch_case_insensitiveを分岐させる

    //環境変数のセットの仕方：
    //  PowerShell: $Env:IGNORE_CASE=1; cargo run -- 検索語 poem.txt
    //  Unix/bash:  IGNORE_CASE=1 cargo run -- 検索語 poem.txt

    let result2 = search_case_insensitive("rUsT", "Rust:\nTrust me.");
    println!("大小無視検索: {:?}", result2); // ["Rust:", "Trust me."]


    // ========================================
    // 12-6: 標準出力ではなく標準エラーにエラーメッセージを書き込む
    // ========================================

    //端末の2種類の出力ストリーム：
        //標準出力 (stdout): println!マクロ  → 正常な出力
        //標準エラー (stderr): eprintln!マクロ → エラーメッセージ

    //使い分けの理由：
    //  $ cargo run > output.txt のようにリダイレクトすると
    //  println!の出力はファイルに行くが、eprintln!の出力は画面に残る
    //  → エラーメッセージは画面で確認しつつ、結果だけファイルに保存できる

    eprintln!("これはエラーメッセージ（画面に出る）");
    println!("これは正常出力（ファイルにリダイレクト可能）");


    // ========================================
    // 12章まとめ：プロジェクト構成のベストプラクティス
    // ========================================

    //最終的なプロジェクト構造：
    //  minigrep/
    //  ├── src/
    //  │   ├── main.rs    ← 引数解析、Config構築、run呼び出し、エラー処理
    //  │   └── lib.rs     ← Config構造体、build、run、search関数など全ロジック
    //  ├── tests/         ← 結合テスト
    //  └── poem.txt       ← テスト用ファイル

    //学んだ新しい概念：
    //  - std::env::args()         コマンドライン引数の取得
    //  - fs::read_to_string()     ファイル読み込み
    //  - Box<dyn Error>           トレイトオブジェクトによる柔軟なエラー型
    //  - unwrap_or_else(|err|{})  エラー時のカスタム処理
    //  - process::exit()          プロセスの終了
    //  - .lines()                 文字列を行ごとに分割するイテレータ
    //  - .contains()              部分文字列の検索
    //  - .to_lowercase()          文字列の小文字化
    //  - env::var()               環境変数の読み取り
    //  - eprintln!                標準エラーへの出力
    //  - TDDプロセス              テスト → 実装 → リファクタリング
}
