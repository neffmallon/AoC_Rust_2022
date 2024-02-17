use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 5;

fn read_input_stacks() -> Vec<String>{
    // println!("Reading file");
    let mut stacks:Vec<String> =vec![String::new();9];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            // println!("Day {DAY} Part 1: {:?}",line);
            if !line.contains("["){break} else {
                for i in 0..9{
                    if line.chars().nth(i*4+1).unwrap() == ' '{continue} else {
                        stacks[i].push(line.chars().nth(i * 4 + 1).unwrap())
                    }
                }
            }
        }
    }
    return stacks
}

fn read_move_order(line: &String)->(u8,u8,u8){
    let split_line: Vec<&str> = line.split(" ").collect();
    return (
        split_line[1].parse::<u8>().unwrap(),
        split_line[3].parse::<u8>().unwrap()-1,
        split_line[5].parse::<u8>().unwrap()-1
    )
}

fn make_move_with_rev(stacks: &mut Vec<String>, move_order: (u8, u8, u8)) {
    // move move_order[0] from stacks[move_order[1]] to stacks[move_order[2]]
    let binding = stacks[move_order.1 as usize].clone();
    let (to_move, stay_put) = binding.split_at(move_order.0 as usize);
    let rev_moved_boxes = to_move.chars().rev().collect::<String>();
    stacks[move_order.2 as usize] = format!("{}{}",rev_moved_boxes,stacks[move_order.2 as usize]);
    stacks[move_order.1 as usize] = stay_put.to_string();
}

pub(crate) fn part_1() {
    let mut stacks:Vec<String> = read_input_stacks();
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if !line.contains("move") { continue } else {
                let move_order = read_move_order(&line);
                make_move_with_rev(&mut stacks, move_order);
            }
        }
    }
    println!("Day {DAY} Part 1: {:?}",
             stacks.iter()
                 .map(|s| s.chars().nth(0).unwrap())
                 .collect::<String>()
    )
}

fn make_move_no_rev(stacks: &mut Vec<String>, move_order: (u8, u8, u8)) {
    // move move_order[0] from stacks[move_order[1]] to stacks[move_order[2]]
    let binding = stacks[move_order.1 as usize].clone();
    let (to_move, stay_put) = binding.split_at(move_order.0 as usize);
    stacks[move_order.2 as usize] = format!("{}{}",to_move.to_string(),stacks[move_order.2 as usize]);
    stacks[move_order.1 as usize] = stay_put.to_string();
}


pub(crate) fn part_2() {
    let mut stacks:Vec<String> = read_input_stacks();
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if !line.contains("move") { continue } else {
                let move_order = read_move_order(&line);
                make_move_no_rev(&mut stacks, move_order);
            }
        }
    }
    println!("Day {DAY} Part 2: {:?}",
             stacks.iter()
                 .map(|s| s.chars().nth(0).unwrap())
                 .collect::<String>()
    )
}