use std::collections::HashSet;
use crate::general_helpers::read_day_input_lines;
use std::cmp::Ordering;


const DAY: u8 = 9;
// problem at https://adventofcode.com/2022/day/9

fn new_tail_position(h_i: i32, h_j: i32, t_i: i32, t_j: i32) -> (i32, i32){
    let mut new_i = t_i;
    let mut new_j = t_j;
    if ((h_i - t_i).abs() <= 1) & ((h_j - t_j).abs() <= 1){return (new_i, new_j)}
    else if ((h_i - t_i).abs() > 1) & ((h_j - t_j).abs() > 1){
        if h_i > t_i {new_i += 1} else {new_i -= 1};
        if h_j > t_j {new_j += 1} else {new_j -= 1};
        // panic!("Head is too far from tail!")
    }
    else if (h_i - t_i).abs() > 1 {
        if h_i > t_i {new_i += 1} else {new_i -= 1};
        match h_j.cmp(&t_j) {
            Ordering::Less => new_j -= 1,
            Ordering::Greater => new_j += 1,
            Ordering::Equal => return (new_i, new_j),
        }
    }
    else if (h_j - t_j).abs() > 1 {
        if h_j > t_j {new_j += 1} else {new_j -= 1};
        match h_i.cmp(&t_i) {
            Ordering::Less => new_i -= 1,
            Ordering::Greater => new_i += 1,
            Ordering::Equal => return (new_i, new_j),
        }
    }

    (new_i, new_j)
}

pub(crate) fn part_1() {
    let (mut head_i, mut head_j) = (0, 0);
    let (mut tail_i, mut tail_j) = (0, 0);
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert((tail_i, tail_j));
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            let (direction, distance) = line.trim().split_at(line.find(' ').unwrap());
            for _ in 0..distance.trim().parse().unwrap(){
                match direction {
                    "U" => head_j += 1,
                    "D" => head_j -= 1,
                    "L" => head_i -= 1,
                    "R" => head_i += 1,
                    _ => panic!(),
                }
                (tail_i, tail_j) = new_tail_position(head_i, head_j, tail_i, tail_j);
                tail_visited.insert((tail_i, tail_j));
            }
        }
    }
    println!("Day {DAY} part 1: tail visited {} spots", tail_visited.len());
    //println!("Day {DAY} part 1: tail visited {:?} spots", tail_visited);
}

pub(crate) fn part_2() {
    let mut rope = [(0,0); 10];
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert((rope[9].0, rope[9].1));
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            let (direction, distance) = line.trim().split_at(line.find(' ').unwrap());
            for _ in 0..distance.trim().parse().unwrap(){
                match direction {
                    "U" => rope[0].1 += 1,
                    "D" => rope[0].1 -= 1,
                    "L" => rope[0].0 -= 1,
                    "R" => rope[0].0 += 1,
                    _ => panic!(),
                }
                for idx in 0..9 {
                    let (tail_i, tail_j) = new_tail_position(rope[idx].0, rope[idx].1, rope[idx+1].0, rope[idx+1].1);
                    if (tail_i == rope[idx + 1].0) & (tail_j == rope[idx + 1].1) {
                        break
                    } else {
                        rope[idx + 1].0 = tail_i;
                        rope[idx + 1].1 = tail_j;
                    }
                }
                tail_visited.insert((rope[9].0, rope[9].1));
            }
            println!("Head Position {}, {}", rope[0].0, rope[0].1);
            println!("5 Position {}, {}", rope[5].0, rope[5].1);
            println!("tail Position {}, {}", rope[9].0, rope[9].1);
        }
    }
    println!("Day {DAY} part 1: tail visited {} spots", tail_visited.len());
}