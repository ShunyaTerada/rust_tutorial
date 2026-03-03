// RefCell<T>と内部可変性（Interior Mutability）パターン
    // 不変参照(&T)しか持っていない状態でも、内部のデータを変更できるデザインパターン
    // 通常、Rustの借用ルールでは「不変参照がある間は可変参照を持てない」が、
    // RefCell<T>を使うとこのチェックを「コンパイル時」ではなく「実行時」に遅延させることができる。

    // なぜ必要か？
    // メソッドのシグネチャ上は `&self` (不変参照) を要求されているが、
    // 実装の都合上、内部状態(例えばテストにおけるモックの呼び出し履歴など)を書き換えたい場合があるため。

    // 注意点:
    // 実行時に借用ルール(可変参照は同時に1つだけ、不変と可変は混在不可)を破ると `panic!` が発生する。
    // Rc<T>と同様、RefCell<T>もシングルスレッド専用。

pub trait Messenger {
    fn send(&self, msg: &str); // 不変参照(&self)を要求している
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("エラー：割り当てを超えています");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("緊急警告：割り当ての90%以上を使用してしまいました");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("警告：割り当ての75%以上を使用してしましました");
        }
    }
}

// モックオブジェクトをインスタンスとして生成し、set_valueメソッドのふるまいをテストしたい
// 本番のMessengerの実装ではなく、テスト用にメッセージを記録するMockMessengerを作る

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // メッセージの履歴を保存したいが、Messengerトレイトのsendメソッドは引数に &self (不変) を取る。
        // 不変参照のままで Vec を変更するために RefCell<T> で包む。
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // borrow_mut()メソッドで、RefCell内のデータに対する「可変参照」を実行時に取得する。
            // これにより、外側(self)は不変でも、内側(sent_messages)を変更できる（内部可変性）。
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // 80をセットすると、75%以上なのでsendが1回呼ばれるはず
        limit_tracker.set_value(80);

        // 結果を検証する際も、borrow()で「不変参照」を取得してから要素数を読む
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

fn main() {
    println!("このファイルは主としてモックテストの実装例(RefCell<T>)を示しています。");
    println!("`cargo test --bin 15_5ReCell` でテストを実行できます。");
}