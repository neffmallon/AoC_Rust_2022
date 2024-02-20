use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 6;

fn check_for_dups(s: &str) ->bool{
    let mut check_str = String::new();
    for c in s.chars(){
        if check_str.contains(c){
            return false
        } else{
            check_str.push(c)
        }
    }
    true
}

pub(crate) fn part_1() {
    let mut signal:String = String::new();
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            signal = line.trim().to_string()
        }
    }
    for i in 0..signal.len()-4{
        if check_for_dups(&signal[i..i+4]){
            println!("Day {DAY} Part 1: {} at {:?}", &signal[i..i+4], i+4); break;
        } else {continue}
    }
    //println!("Day {DAY} Part 1: {:?}",check_for_dups("asdsf")
}

pub(crate) fn part_2() {
    let mut signal:String = String::new();
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            signal = line.trim().to_string()
        }
    }
    for i in 0..signal.len()-14{
        if check_for_dups(&signal[i..i+14]){
            println!("Day {DAY} Part 2: {} at {:?}", &signal[i..i+14], i+14); break;
        } else {continue}
    }
    //println!("Day {DAY} Part 1: {:?}",check_for_dups("asdsf")
}