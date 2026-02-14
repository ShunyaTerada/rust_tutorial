#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//構造体のふるまいを定義
impl Rectangle { //implementation（実装）の意
    fn area(&self) -> u32 { //self: &Rectangleの省略記法
        self.width * self.height
    }

    fn width(&self) -> bool { //フィールド名と同じメソッド名も定義できる
        self.width > 0
    }

    //2番目のRectangleがself（1番目のRectangle）に完全に収まるなら、trueを返す
    fn can_hold(&self, str: &Rectangle) -> bool{
        self.width > str.width && self.height > str.height
    }

    //メソッドでない関連関数は、新規インスタンスを返すconstructorによく使用される
    fn square(size: u32) -> Self{ 
        Self {
            width: size,
            height: size,
        }
    }
        //使用するときはString型のようにRectangle::square(3)という感じでつかう
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    if rect1.width() { 
        println!("長方形の面積は0ではありません");
    }else {
        println!("長方形の面積は0です。");
    }
    println!("\n====================");

    println!(
        "長方形の面積は{}平方ピクセルです",
        rect1.area()
    );
    println!("\n====================");

    println!(
        "rect1はrect2をホールドするか？：{}", rect1.can_hold(&rect2)
    );

    println!(
        "rect1はrect3をホールドするか？：{}", rect1.can_hold(&rect3)
    );
    println!("\n====================");

    let x = 5;
    let y = Rectangle::square(x).area();
    println!(
        "一辺が{}の長方形の面積は{}です",
        x,
        y,
    );

    


    
}