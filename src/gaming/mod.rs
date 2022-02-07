mod io;
mod validation;
mod word_serving;

use word_serving::{answer::Answer, answers::Answers};
use io::{output_string::OutputString, InputAnswer};
use validation::ValidateAnswer;

pub fn run() {
    println!("\nGame Start!\n");

    let answer = word_serving::serving::pick_word();

    let mut user_string = word_serving::answer::AnswerType::new();
    let mut answers = word_serving::answers::AnswersType::new();

    while run_phase(&mut answers, answer, &mut user_string) {}
}

fn run_phase(
    answers: &mut word_serving::answers::AnswersType,
    answer: word_serving::answer::AnswerType,
    user_string: &mut word_serving::answer::AnswerType
) -> bool {
    answers.make_output_string(answer).print();
    match answers.set_answer(user_string.input()).validate(answer) {
        Some(_) => {
            io::output_string::clear_game_console();
            answers.make_output_string(answer).print();
            false
        },
        None => {
            io::output_string::clear_game_console();
            true
        }
    }
}