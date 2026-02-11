use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("数字を予想してください！");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("予想を書き込もう！");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み込みに失敗しました");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("あなたの予想：{guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("小さすぎます"),
            Ordering::Greater => println!("大きすぎます"),
            Ordering::Equal => {
                println!("あたりです！");
                break;
            },
        }

        println!("========================");  
    }   
}

