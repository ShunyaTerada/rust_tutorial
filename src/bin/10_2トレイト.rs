//トレイト：型のふるまいをグループにまとめて共通化する
    //他の言語ではインターフェイスと呼ばれる
    //implのようにtraitキーワードを使用する
    //メソッドの実装（{}内の記述）は定義せず、シグネチャだけを定義できる

fn main() {
    //異なる文書構造体（ArticleやTweetなど）の要約を出力したい

    //トレイトの作成
    pub trait Summary {//これに依存するクレートに公開するためにpubをつける
        fn summarize(&self) -> String;//具体的な実装は個別のメソッドに任せる
    }

    //型に実装
    pub struct Article { 
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for Article { //forキーワードで適用
        fn summarize(&self) -> String { //規定のシグネチャを使用
            format!("{}, by {} ({})",
                self.headline,
                self.author,
                self.location,
            )
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    //ユースケース
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "新製品発表！！"
        ),
        reply: false,
        retweet: false,
    };

    println!("new tweet: {}", tweet.summarize());

    // 注意すべき制限の1つは、トレイトか、対象の型のうち、少なくとも一方が自分のクレートに固有(local)である時のみ、 型に対してトレイトを実装できるということです。
    // 例えば、Displayのような標準ライブラリのトレイトをaggregatorクレートの機能の一部として、 Tweetのような独自の型に実装できます。
    // 型Tweetがaggregatorクレートに固有だからです。 
    // また、SummaryをaggregatorクレートでVec<T>に対して実装することもできます。 
    // トレイトSummaryは、aggregatorクレートに固有だからです。
    // 外部のトレイトを外部の型に対して実装することはできません。

    println!("\n=======================");


    //デフォルト実装
    pub trait Summary2 {
        fn summarize2(&self) -> String {
            String::from("（もっと読む）") //デフォルトのふるまい
        }
    }

    impl Summary2 for Article {} //空の{}でデフォルト実装が適用される

    let article = Article {
        headline: String::from( "甲子園優勝！"),
        location: String::from("アメリカ, PA, USA"),
        author: String::from("アイスバーグ"),
        content: String::from("再び甲子園優勝！"),
    };

    println!(
        "新しい記事があります！ {}",
        article.summarize2()
    );
    println!("\n=======================");


    //他のメソッドの呼び出し
        //自らのトレイトのデフォルト実装を持ってないときに可能
    pub trait Summary3 {
        fn summarize_author(&self) -> String; //実装させる部分

        fn summarize3(&self) -> String { //実装させた部分を使ったデフォルト実装
            format!("(もっと読む{}...)", self.summarize_author())
        }
    }

    impl Summary3 for Tweet {
        fn summarize_author(&self) -> String { //すでにデフォルメされている箇所があるので、一部だけを実装すればいい
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("新しい投稿があります！：{}", tweet.summarize3());
    println!("\n=======================");


    //引数としてのトレイト
    pub fn notify(item: &impl Summary3) { //引数の型をトレイトの参照にする
    //pub fn notify<T: Summary3>(item: &T) の糖衣構文
        println!("速報！ {}", item.summarize3());//メソッドが使用できる
    }
    notify(&tweet);

        //複数の引数の型を同じトレイトに強制したい場合
    pub fn notify_multiarg<T: Summary3>(item1: &T, item2: &T){
        println!("二つの速報！ １：{}、２：{}", item1.summarize3(), item2.summarize3())
    }


        //複数の境界を+構文で指定
    use std::fmt::Display;
    pub fn notify_multiple_interferce<T: Summary + Display>(item: &T) {
        //略
    }


        //where句を使った明確なトレイト境界

            //冗長な例
    use core::fmt::Debug;
    fn some_fn<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

            //where版
    fn some_fn2<T, U>(t: &T, u: &U)
    where   // ジェネリクスの内容を引数部分と引き離して定義できる
        T: Display + Clone,
        U: Clone + Debug,
    {}


    //トレイトを実装している型を返す
    fn returns_summarizable() -> impl Summary3 {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }


    //トレイト境界を使用して、メソッド実装を条件分けする
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> { //無条件
        fn new(x: T, y: T) -> Self { //Selfはimplブロックの型に対する型エイリアス
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> { //条件あり
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("最大値はｘです {}", self.x);
            }else {
                println!("最大値はｙです {}", self.y);
            }
        }
    }


    //ブランケット実装
    // Diplayトレイトを実装するすべての型に共通メソッドを定義できる
    
    // impl<T: Display> ToString for T {}
    
}   


