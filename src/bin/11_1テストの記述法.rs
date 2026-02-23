fn main() {} //テストはmain関数外に書く

#[cfg(test)]
mod tests {
    use super::*;// 外部モジュール内のテスト配下にあるコードを内部モジュールのスコープに持ってくる

    #[test]
    fn exploration() {
        // let result = add(2, 2);
        assert_eq!(2 + 2, 4);
        assert_ne!(1 + 1, 3);
    }

    #[test]
    fn another() {
        // panic!("失敗テスト");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]//  比較対象の値はPartialEqとDebugトレイトを実装していなければならない
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}