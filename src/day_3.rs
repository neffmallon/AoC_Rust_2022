use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 7;

pub(crate) fn part_1() {
    let mut total_priority: u32 = 0;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            let sack: Vec<u32> = line
                .trim()
                .chars()
                .map(
                    |c| if { c as u32 } < { 'a' as u32 }{
                        27 + c as u32 - 'A' as u32
                    } else{1 + c as u32 - 'a' as u32}
                ).collect();
            let (a,b) = sack.split_at(sack.len()/2);
            'inner : for thing in a {
                if b.contains(thing) {
                    total_priority += thing;
                    break 'inner;
                }
            }
        }
        println!("Day {DAY} Part 1: {total_priority}")
    }
}

fn two_string_overlap<'a>(a: &'a String, b:&'a String) -> String {
    let mut overlap_str = String::new();
    for thing in a.chars(){
        if b.contains(thing){
            overlap_str = format!("{}{}", overlap_str.to_string(), thing.to_string())
        }
    }
    overlap_str
}

fn three_string_overlap(v: &[String; 3]) -> String {
    let first_overlap = two_string_overlap(&v[0],&v[1]);
    two_string_overlap(&first_overlap,&v[2])
}

pub(crate) fn part_2() {
    let mut total_priority: u32 = 0;
    let mut team_idx = 0;
    let mut team_array = [String::new(),String::new(),String::new()];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            team_array[team_idx] = line.trim().to_string();
            team_idx += 1;
            if team_idx == 3 {
                let c = three_string_overlap(&team_array).chars().collect::<Vec<char>>()[0];
                total_priority += if { c as u32 } < { 'a' as u32 }{
                        27 + c as u32 - 'A' as u32
                    } else{1 + c as u32 - 'a' as u32};
                team_idx = 0;
                continue
            }
        }
    }
    println!("Day {DAY} Part 2: {}",total_priority)
}