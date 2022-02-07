use super::{answer, answer::*};
use super::words;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn pick_word() -> answer::AnswerType {
    let now = if let Ok(v) = SystemTime::now().duration_since(UNIX_EPOCH) {
        v.as_secs()
    } else {
        0
    };
    let today = now - now % 86400;
    answer::AnswerType::make_answer(String::from(words::WORDS[(StdRng::seed_from_u64(today).gen::<f32>() *  (words::WORDS.len() as f32)) as usize]))
}
