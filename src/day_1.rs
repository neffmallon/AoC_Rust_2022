use crate::general_helpers::read_day_input_lines;
use std::cmp::Ordering;

const DAY: u8 = 1;


fn min(input_array: &[i32]) -> &i32 {
    let x = input_array.iter().min().unwrap();
    x
}

pub(crate) fn part_1() {
    let mut max_food = 0;
    let mut this_food = 0;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.to_string().trim() == "" {
                match this_food.cmp(&max_food) {
                    Ordering::Less => {
                        this_food = 0
                    },
                    Ordering::Greater => {
                        max_food = this_food;
                        this_food = 0;
                    },
                    Ordering::Equal => this_food = 0,
                }
            } else {
                let a_food: u32 = match line.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                this_food += a_food
            }
        }
        println!("max food: {max_food}")
    }
}

pub(crate) fn part_2() {
    let mut max_foods = vec![0,0,0];
    let mut this_food = 0;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.to_string().trim() == "" {
                let min_val = min(&max_foods);
                match this_food.cmp(min_val) {
                    Ordering::Less => {
                        this_food = 0
                    },
                    Ordering::Greater => {
                        let index = max_foods.iter().position(|r| r == min_val).unwrap();
                        max_foods[index] = this_food;
                        this_food = 0;
                    },
                    Ordering::Equal => this_food = 0,
                }
            } else {
                let a_food: i32 = match line.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                this_food += a_food
            }
        }
        let food_sum = max_foods[0] + max_foods[1] + max_foods[2];
        println!("max food sum: {food_sum}");
    }
}