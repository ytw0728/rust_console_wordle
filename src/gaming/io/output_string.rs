use super::super::word_serving::{answer, answers};
use super::GetPrintColor;
use super::{ConsoleColor, ConsoleCursor};
use std::io::{Write, stdout};

pub fn clear_game_console() {
    let mut std_out = stdout();
    std_out.write_all(ConsoleCursor::Base.to_string().as_bytes()).unwrap();
    std_out.flush().unwrap();
}

pub struct OutputStringBuffer {
    pub message: String
}
impl OutputStringBuffer {
    pub fn print(&self) -> () {
        let mut std_out = stdout();
        std_out.write_all(self.message.as_bytes()).unwrap();
        std_out.flush().unwrap();
    }
}

pub trait OutputString {
    fn make_output_string(&self, answer: answer::AnswerType) -> OutputStringBuffer;
}
impl OutputString for answers::AnswersType {
    fn make_output_string(&self, _answer: answer::AnswerType) -> OutputStringBuffer {
        let mut output = String::from("");

        for i in 0..6 {
            output.push_str(ConsoleCursor::ClearLine.to_string().as_str());
            output.push_str(self[i].map(
                |item|
                if item.get_value() == ' ' {
                    String::from("ㅁ")
                } else {
                    format!(
                        "{color}[ {value} ]{default}",
                        color = item.get_print_color(),
                        value = item.get_value(),
                        default = ConsoleColor::Default,
                    )
                }
            ).concat().as_str());
            output.push_str("\n");
        }
        output.push_str("\n");

        OutputStringBuffer {
            message: output
        }
    }
}