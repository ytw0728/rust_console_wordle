use super::answer_item;

pub type AnswerType = [answer_item::AnswerItem; 5];
pub trait Answer {
    fn new () -> Self;
    fn make_answer (answer: String) -> Self;
    fn inserted(self) -> bool;
}

impl Answer for AnswerType {
    fn new() -> Self {
        [ answer_item::AnswerItem::new(), answer_item::AnswerItem::new(), answer_item::AnswerItem::new(), answer_item::AnswerItem::new(), answer_item::AnswerItem::new() ]
    }

    fn make_answer(answer: String) -> Self {
        let mut word = AnswerType::new();
        for (i, item) in word.iter_mut().enumerate() {
            *item = answer_item::AnswerItem::Default(answer.chars().nth(i).unwrap());
        }
        word
    }

    fn inserted(self) -> bool {
        self[0].get_value() != ' '
    }
}