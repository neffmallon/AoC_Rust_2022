use std::cmp::Ordering;
use colored::{ColoredString, Colorize};
use crate::general_helpers::read_day_input_lines;


const DAY: u8 = 14;
// problem at https://adventofcode.com/2022/day/14

fn split_tuple(s: &str)-> (usize, usize){
    let vec: Vec<usize> = s.trim().split(",")
        .into_iter()
        .map(|sub| sub.parse::<usize>().unwrap())
        .collect();
    (vec[0], vec[1])
}

fn wall_inst_to_vec_tuples(s: &String)-> Vec<(usize, usize)>{
    s.split("->").into_iter().map(split_tuple).collect()
}

fn find_cave_bounds()-> (usize, usize, usize){
    let mut min_x = 500usize;
    let mut max_x = 500usize;
    let mut max_y = 0usize;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            let wall = wall_inst_to_vec_tuples(&line);
            for p in wall.iter(){
                if p.0 > max_x{ max_x = p.0 }
                if p.0 < min_x{ min_x = p.0 }
                if p.1 > max_y{ max_y = p.1 }
            }
        }
    }
    return (min_x, max_x, max_y)
}

fn build_infinite_cave_map(empty: &ColoredString, wall: &ColoredString, bounds: (usize, usize, usize)) -> Vec<Vec<ColoredString>>{
    let mut cave_map = vec![vec![empty.clone();bounds.1-bounds.0+1];bounds.2+1];
        if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            let wall_inst = wall_inst_to_vec_tuples(&line);
            for stop_idx in 1..wall_inst.len(){
                let start = wall_inst[stop_idx-1];
                let stop = wall_inst[stop_idx];
                let x_range = match start.0.cmp(&stop.0){
                    Ordering::Equal => start.0..(start.0+1),
                    Ordering::Greater => stop.0..(start.0+1),
                    Ordering::Less => start.0..(stop.0+1),
                };
                let y_range = match start.1.cmp(&stop.1){
                    Ordering::Equal => start.1..(start.1+1),
                    Ordering::Greater => stop.1..(start.1+1),
                    Ordering::Less => start.1..(stop.1+1),
                };
                for x in x_range{
                    for y in y_range.clone(){
                        cave_map[y-1][x-bounds.0] = wall.clone();
                    }
                }
            }
        }
    }
    return cave_map
}

fn build_finite_cave_map(empty: &ColoredString, wall: &ColoredString, bounds: (usize, usize, usize)) -> Vec<Vec<ColoredString>>{
    let mut cave_map = vec![vec![empty.clone();bounds.1-bounds.0+1];bounds.2+1];
        if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            let wall_inst = wall_inst_to_vec_tuples(&line);
            for stop_idx in 1..wall_inst.len(){
                let start = wall_inst[stop_idx-1];
                let stop = wall_inst[stop_idx];
                let x_range = match start.0.cmp(&stop.0){
                    Ordering::Equal => start.0..(start.0+1),
                    Ordering::Greater => stop.0..(start.0+1),
                    Ordering::Less => start.0..(stop.0+1),
                };
                let y_range = match start.1.cmp(&stop.1){
                    Ordering::Equal => start.1..(start.1+1),
                    Ordering::Greater => stop.1..(start.1+1),
                    Ordering::Less => start.1..(stop.1+1),
                };
                for x in x_range{
                    for y in y_range.clone(){
                        cave_map[y][x-bounds.0] = wall.clone();
                    }
                }
            }
        }
    }
    cave_map.push(vec![empty.clone();bounds.1-bounds.0+1]);
    cave_map.push(vec![wall.clone();bounds.1-bounds.0+1]);
    return cave_map
}

fn fill_with_sand(cave_map: &mut Vec<Vec<ColoredString>>, sand_start_idx: usize, empty: &ColoredString, sand: &ColoredString) -> u32 {
    let mut sand_count = 0u32;
    let mut keep_adding = true;
    let cave_depth = cave_map.len();
    while keep_adding{
        let mut keep_falling = true;
        let mut sand_loc = (sand_start_idx, 0usize);
        while keep_falling{
            if sand_loc.1 + 1 >= cave_depth{
                keep_adding=false; break  // we have reached the bottom!
            }
            else if cave_map[sand_loc.1+1][sand_loc.0] == *empty{
                sand_loc = (sand_loc.0, sand_loc.1+1)  // move directly down
            }
            else if sand_loc.0 == 0{
                keep_adding=false; break  // we have reached the left edge!
            }
            else if cave_map[sand_loc.1+1][sand_loc.0-1] == *empty{
                sand_loc = (sand_loc.0-1, sand_loc.1+1)  // move left down
            }
            else if sand_loc.0 + 1 == cave_map[0].len(){
                keep_adding=false; break // we have reached the right edge!
            }
            else if cave_map[sand_loc.1+1][sand_loc.0+1] == *empty{
                sand_loc = (sand_loc.0+1, sand_loc.1+1)  // move right down
            }
            else if sand_loc == (sand_start_idx, 0usize) {
                cave_map[sand_loc.1][sand_loc.0] = sand.clone();
                sand_count+=1;
                keep_adding=false;
                break; // The top is clogged!
            }
            else { // stop moving, add more sand!
                cave_map[sand_loc.1][sand_loc.0] = sand.clone();
                sand_count+=1;
                keep_falling = false
            }
        }
    }
    return sand_count
}

pub(crate) fn part_1() {
    // define display strings:
    let empty = ".".white();
    let wall = "W".red();
    let sand = "s".yellow();
    let bounds = find_cave_bounds();
    println!("Bounds: {:?}", bounds);
    // build the cave map
    let mut cave_map = build_infinite_cave_map(&empty, &wall, bounds);
    // fill it with sand!
    let sand_count = fill_with_sand(&mut cave_map, 500-bounds.0, &empty, &sand);
    // print the cave map
    // for line in cave_map.into_iter(){
    //     for c in line[0..line.len()].iter(){
    //         print!("{}", c)
    //     }
    //     print!("\n")
    // }
    println!("Day {DAY} Part 1: {sand_count}");
}

pub(crate) fn part_2() {
    // define display strings:
    let empty = ".".white();
    let wall = "W".red();
    let sand = "s".yellow();
    let wall_bounds = find_cave_bounds();
    let bounds = (500 - (wall_bounds.2 + 10), 500 + (wall_bounds.2 +10) , wall_bounds.2);
    // build the cave map
    let mut cave_map = build_finite_cave_map(&empty, &wall, bounds);
    // fill it with sand!
    let sand_count = fill_with_sand(&mut cave_map, 500-bounds.0, &empty, &sand);
    // print the cave map
    for line in cave_map.into_iter(){
        for c in line[(wall_bounds.0 - bounds.0 - 1)..(2 + wall_bounds.1 - bounds.0)].iter(){
            print!("{}", c)
        }
        print!("\n")
    }
    println!("Day {DAY} Part 2: {sand_count}");
}
