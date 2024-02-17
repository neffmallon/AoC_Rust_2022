use std::collections::HashSet;
use crate::general_helpers::read_day_input_lines;
use std::cmp::Ordering;


const DAY: u8 = 10;
// problem at https://adventofcode.com/2022/day/10

fn check_cycle(x: &i32, cycle_count: &i32, next_cycle_check: &i32, final_sum: &i32)-> (i32, i32){
    if cycle_count == next_cycle_check{
        println!("{} added to total for a new sum of {}", x * cycle_count, final_sum + x * cycle_count);
        return (next_cycle_check + 40, final_sum + x * cycle_count)
    }
    return (next_cycle_check.clone(), final_sum.clone())
}

pub(crate) fn part_1() {
    let mut x = 1;
    let mut cycle_count = 1;
    let mut next_cycle_check = 20;
    let mut final_sum = 0;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.trim() == "noop"{
                // cycle start
                (next_cycle_check, final_sum) = check_cycle(&x, &cycle_count, &next_cycle_check, &final_sum);
                cycle_count += 1; // cycle end
            } else if &line.trim()[0..4] =="addx"{
                // cycle start
                (next_cycle_check, final_sum) = check_cycle(&x, &cycle_count, &next_cycle_check, &final_sum);
                cycle_count += 1;// cycle end
                // cycle start
                (next_cycle_check, final_sum) = check_cycle(&x, &cycle_count, &next_cycle_check, &final_sum);
                cycle_count += 1; // cycle end
                let n =  match line.trim()
                    .split_at(line.find(" ").unwrap()).1.trim()
                    .parse::<i32>()
                { Ok(num) => num,
                Err(_) => {
                    println!("{}", line);
                    panic!()
                },};
                x += n;
            }
        if next_cycle_check > 220{break}
        }
    }
    println!("Day {DAY} part 1: {}",final_sum);
}

pub(crate) fn part_2() {
    println!("Day {DAY} part 2: incomplete");
}