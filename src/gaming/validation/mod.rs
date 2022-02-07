use std::collections::HashMap;
use super::word_serving::{answer_item, answer, answers, answers::Answers};

pub struct Finished;

pub trait ValidateAnswer {
    fn validate(&mut self, answer: answer::AnswerType) -> Option<Finished>;
}

impl ValidateAnswer for answers::AnswersType {
    /* 이 함수는 찌들었다. 분리를 위한 열정의 여백이 부족해 작성하지 않는다. */
    fn validate(&mut self, answer: answer::AnswerType) -> Option<Finished> {
        // 전처리
        let mut m: HashMap<char, std::vec::Vec<usize>> = HashMap::new();
        answer.iter().enumerate().for_each(|(i, v)| {
            match m.get_mut(&v.get_value()) {
                Some(list) => list.push(i),
                None => {
                    m.insert(v.get_value(), vec![i]);
                }
            }
        });

        // 초기화
        self.iter_mut().enumerate().for_each(|(_, word)| {
            word.iter_mut().enumerate().for_each(|(_, v)| *v = answer_item::AnswerItem::Default(v.get_value()));
        });

        // 체크
        self.iter_mut().for_each(|word| {
            let mut cloned_map = m.clone();
            // 1) answer 기준 체크
            answer
                .iter().enumerate().for_each(|(i, letter)| {
                    let c = letter.get_value();
                    if word[i].get_value() == c {
                        cloned_map.get_mut(&c).and_then(|positions| {
                            positions.remove(positions.iter().position(|p| *p == i).unwrap());
                            Some(positions)
                        }).unwrap();
                        word[i] = answer_item::AnswerItem::Strike(c);
                    }
                });
            
            // 2) word 기준 체크
            word.iter_mut().enumerate()
            .filter(|(_, v)| if let answer_item::AnswerItem::Strike(_) = v { false } else { true })
            .for_each(|(_, letter)| {
                match cloned_map.get_mut(&letter.get_value()) {
                    Some(positions) => {
                        if positions.len() > 0 {
                            *letter = answer_item::AnswerItem::Ball(letter.get_value());
                        }
                        positions.pop();
                    },
                    None => {},
                }
            });
        });

        // Finished 여부 체크
        if self.inserted_num() == 6 {
            // 1) 입력 다했으면 끝
            Some(Finished)
        } else {
            // 2) 정답 하나라도 있으면 끝
            match
                self.iter().find(|word| {
                    word
                    .map(|letter| if let answer_item::AnswerItem::Strike(_) = letter { 1 } else { 0 })
                    .iter().sum::<i32>()
                    == 5
                })
            {
                Some(_) => Some(Finished),
                None => None
            }
        }
    }
}