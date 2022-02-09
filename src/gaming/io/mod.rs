pub mod output_string;

use super::word_serving::{answer_item, answer, words};
use super::word_serving::answer::*;
use std::io::{Write, stdout};

pub enum ConsoleCursor {
    Base,
    ClearLine,
    Input,
}

impl std::fmt::Display for ConsoleCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ConsoleCursor::Base =>  "\x1B[0G\x1B[8A",
            ConsoleCursor::ClearLine => "\x1B[K",
            ConsoleCursor::Input => "\x1B[10G",
        })
    }
}

pub enum ConsoleColor {
    Green,
    Yellow,
    Default,
}

impl std::fmt::Display for ConsoleColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ConsoleColor::Green => "\x1B[32m",
            ConsoleColor::Yellow => "\x1B[33m",
            ConsoleColor::Default => "\x1B[39m",
        })
    }
}

pub trait GetPrintColor {
    fn get_print_color(self) -> String;
}
impl GetPrintColor for answer_item::AnswerItem {
    fn get_print_color(self) -> String {
        match self {
            answer_item::AnswerItem::Default(_) => ConsoleColor::Default,
            answer_item::AnswerItem::Strike(_) => ConsoleColor::Green,
            answer_item::AnswerItem::Ball(_) => ConsoleColor::Yellow,
        }.to_string()
    }
}

pub trait InputAnswer {
    fn input(&mut self) -> &Self;
}
impl InputAnswer for answer::AnswerType {
    /* input validate도 Validate module로 옮기고 싶은데 열정의 여백이... */
    fn input(&mut self) -> &Self {
        let mut std_out = stdout();
        std_out.write_all(
            format!("{clear_line}정답은 ? {input_cursor}", clear_line = ConsoleCursor::ClearLine, input_cursor = ConsoleCursor::Input).as_bytes()
        ).unwrap();
        std_out.flush().unwrap();

        let mut input = String::new();
        loop {
            if std::io::stdin().read_line(&mut input).is_ok() {
                input = input.to_lowercase();
                break;
            }
        }

        let trimmed_input = input.trim();
        if words::WORDS.iter().any(|word| word.matches(trimmed_input).count() == 1) {
            trimmed_input.chars().enumerate().for_each(|(i, v)| self[i] = answer_item::AnswerItem::Default(v));
        } else {
            *self = answer::AnswerType::new();
        }

        self
    }
}
