use super::answer;
use super::answer::*;

pub type AnswersType = [answer::AnswerType; 6];
pub trait Answers
    where Self: std::marker::Sized {
    fn new() -> AnswersType;
    fn inserted_num(self) -> usize;
    fn set_answer(&mut self, new_answer: &answer::AnswerType) -> &mut Self;
}

impl Answers for AnswersType {
    fn new() -> AnswersType {
        [
            answer::AnswerType::new(),
            answer::AnswerType::new(),
            answer::AnswerType::new(),
            answer::AnswerType::new(),
            answer::AnswerType::new(),
            answer::AnswerType::new(),
        ]
    }

    fn inserted_num(self) -> usize {
        self.iter().position(|v| !v.inserted()).or(Some(6)).unwrap()
    }

    fn set_answer(&mut self, user_input: &answer::AnswerType) -> &mut Self {
        if user_input.inserted() {
            let index = self.inserted_num();
            if index < 6 {
                self[index] = *user_input;
            }
        }
        self
    }
}