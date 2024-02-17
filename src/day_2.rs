use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 2;

pub(crate) fn part_1() {
    let mut total_score = 0;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.to_string().trim() == "" {break};
            total_score += match line.trim() {
                "A X"=> 1 + 3,
                "A Y"=> 2 + 6,
                "A Z"=> 3,
                "B X"=> 1,
                "B Y"=> 2 + 3,
                "B Z"=> 3 + 6,
                "C X"=> 1 + 6,
                "C Y"=> 2,
                "C Z"=> 3 + 3,
                _=>continue,
            }
        }
    }
    println!("Total Score = {total_score}")
}

pub(crate) fn part_2() {
    let mut total_score = 0;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.to_string().trim() == "" { break };
            total_score += match line.trim() {
                "A X" => 3,
                "A Y" => 1 + 3,
                "A Z" => 2 + 6,
                "B X" => 1,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 2,
                "C Y" => 3 + 3,
                "C Z" => 1 + 6,
                _ => continue,
            }
        }
    }
    println!("Total Score = {total_score}")
}