use std::cmp::{min, Ordering};
use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 13;
// problem at https://adventofcode.com/2022/day/13

fn get_closure_index(s: &String) -> usize{
    // Get the index of the string where the '[' in the first position closes.
    // string must start with '['. Find the matching ']'
    let mut closure_state = 0u16;
    let mut current_idx = 0usize;
    if s.as_bytes()[0usize] as char != '['{
        panic!("String must start with '[' to find closure index.")
    } else {closure_state = 1u16;}

    while closure_state > 0{
        current_idx += 1;
        if s.as_bytes()[current_idx] as char == '['{closure_state += 1}
        else if s.as_bytes()[current_idx] as char == ']'{closure_state -= 1}
    }

    return current_idx
}

fn parse_elements(str_to_parse: &String) -> Vec<String>{
    /// Turn the passed string into a vec of strings, one for each element in the top most list
    // if s does not start with '[' then it must be a single number.
    // if that's the case, return the number in a vec
    if !str_to_parse.contains("["){
        return vec![str_to_parse.clone()]
    }
    let mut out: Vec<String> = Vec::new();
    let mut keep_going = true;
    let mut current_idx = 1usize;
    let s = str_to_parse.as_bytes();
    let eos = s.len();

    while keep_going{
        let search_s = s[current_idx..eos].into_iter().map(|n| *n as char).collect::<String>();
        let mut ci = 0usize;
        if s[current_idx] as char == '[' {
            ci = get_closure_index(&search_s) + 1;
        } else if search_s.contains(",") {
            ci = search_s.find(",").unwrap();
        } else { // we should only get here if we are at the last element
            ci = search_s.len() - 1;
        }
        out.push(
            search_s.split_at(ci).0.to_string()
        );
        current_idx +=  ci + 1;
        if current_idx >= eos { keep_going = false}
    }
    return if out == vec![""] { vec![] } else { out }
}

fn compare_packets(lhs: &String, rhs: &String)-> Ordering{
    /// Compare the ordering of the packets in the lhs and rhs
    let lh_compare = parse_elements(lhs);
    let rh_compare = parse_elements(rhs);
    let max_iter = min(lh_compare.len(), rh_compare.len());

    for idx in 0..max_iter{
        if lh_compare[idx].contains("[") || rh_compare[idx].contains("["){
            match compare_packets(&lh_compare[idx], &rh_compare[idx]){
                Ordering::Equal => continue,
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
            }
        } else {
             match lh_compare[idx].parse::<u32>().unwrap()
                .cmp(&rh_compare[idx].parse::<u32>().unwrap()){
                Ordering::Equal => continue,
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
            }
        }
    }

    return lh_compare.len().cmp(&rh_compare.len())
}

pub(crate) fn part_1() {
    let mut line_num = 0u32;
    let mut lhs = String::new();
    let mut idx_sum = 0u32;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            match line_num % 3 {
                0 => lhs = line.clone(),
                1 => {
                    let rhs = line.clone();
                    match compare_packets(&lhs, &rhs){
                        Ordering::Less => idx_sum += 1 + (line_num/3),
                        _ =>{},
                    }
                },
                2 => {},
                _ => panic!()
            };
            line_num += 1
        }
    }
    println!("Day {DAY} part 1: {}", idx_sum);
}

pub(crate) fn part_2() {
    let two = "[[2]]".to_string();
    let six = "[[6]]".to_string();
    let mut current_two_idx = 1u32;
    let mut current_six_less_two_idx = 1u32;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.contains("["){
                match compare_packets(&line, &two){
                    Ordering::Less => current_two_idx += 1,
                    Ordering::Greater => {
                        match compare_packets(&line, &six) {
                            Ordering::Less => current_six_less_two_idx += 1,
                            Ordering::Equal => panic!("No packet should equal [[6]]"),
                            _ => {},
                        }
                    }
                    Ordering::Equal => panic!("No packet should equal [[2]]")
                }
            }

        }
    }
    println!("Day {DAY} Part 2: {}", current_two_idx * (current_two_idx+current_six_less_two_idx));
}