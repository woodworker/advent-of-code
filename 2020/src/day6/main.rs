#[path = "../common.rs"]
mod common;

use std::collections::HashMap;

fn main() {
    let mut sum = 0;
    if let Ok(lines) = self::common::read_lines("./src/day6/input.txt") {
        let mut questions = HashMap::new();
        for line in lines {
            if let Ok(questionline) = line {
                if questionline == "" {
                    sum = sum + questions.len();
                    questions = HashMap::new();
                } else {
                    for c in questionline.chars() {
                        questions.insert(c, true);
                    }
                }
            }
        }
        sum = sum + questions.len();
    }

    println!("Sum: {}", sum);
}
