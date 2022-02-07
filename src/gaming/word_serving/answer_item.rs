#[derive(Debug, Copy, Clone)]
pub enum AnswerItem {
    Default(char),
    Strike(char),
    Ball(char)
}

impl AnswerItem {
    pub fn new () -> AnswerItem {
        AnswerItem::Default(' ')
    }

    pub fn get_value(self) -> char {
        match self {
            AnswerItem::Default(x) => x,
            AnswerItem::Strike(x) => x,
            AnswerItem::Ball(x) => x,
        }
    }
}
