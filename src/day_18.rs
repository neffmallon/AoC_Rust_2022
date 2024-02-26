use std::collections::VecDeque;
use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 18;
// problem at https://adventofcode.com/2022/day/18
// Minecraft, basically

const SIZE: usize = 28;

fn read_ordered_pair(s: &str)->(usize, usize, usize){
    let v: Vec<usize> = s.split(',').map(|n| n.parse::<usize>().unwrap()).collect();
    (v[0]+2, v[1]+2, v[2]+2)
}

fn count_neighbors(op: (usize, usize, usize), all_rocks: &[[[bool;SIZE];SIZE];SIZE]) -> u32{
    let mut out = 0;
    if all_rocks[op.0 + 1][op.1][op.2]{out += 1}
    if all_rocks[op.0][op.1 + 1][op.2]{out += 1}
    if all_rocks[op.0][op.1][op.2 + 1]{out += 1}
    if all_rocks[op.0 - 1][op.1][op.2]{out += 1}
    if all_rocks[op.0][op.1 - 1][op.2]{out += 1}
    if all_rocks[op.0][op.1][op.2 - 1]{out += 1}
    return out
}

pub(crate) fn part_1() {
    let mut all_rocks = [[[false; SIZE]; SIZE]; SIZE];
    let mut surface_area = 0u32;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            let op = read_ordered_pair(&line.trim());
            all_rocks[op.0][op.1][op.2] = true;
            surface_area += 6;
            surface_area -= count_neighbors(op, &all_rocks)*2;
        }
    }
    println!("Day {DAY} Part 1: {surface_area}");
}

fn count_neighbors_int(op: (usize, usize, usize), all_rocks: &[[[u8;SIZE];SIZE];SIZE]) -> u32{
    let mut out = 0;
    if all_rocks[op.0 + 1][op.1][op.2] == 1 {out += 1}
    if all_rocks[op.0][op.1 + 1][op.2] == 1 {out += 1}
    if all_rocks[op.0][op.1][op.2 + 1] == 1 {out += 1}
    if all_rocks[op.0 - 1][op.1][op.2] == 1 {out += 1}
    if all_rocks[op.0][op.1 - 1][op.2] == 1 {out += 1}
    if all_rocks[op.0][op.1][op.2 - 1] == 1 {out += 1}
    return out
}

fn color_neighbors_int(op: (usize, usize, usize), all_rocks: &mut [[[u8;SIZE];SIZE];SIZE])-> VecDeque<(usize, usize, usize)>{
    let mut out = VecDeque::new();
    if op.0 + 1 < SIZE{if all_rocks[op.0 + 1][op.1][op.2] == 0 {all_rocks[op.0 + 1][op.1][op.2] = 2; out.push_back((op.0 + 1,op.1,op.2))}}
    if op.1 + 1 < SIZE{if all_rocks[op.0][op.1 + 1][op.2] == 0 {all_rocks[op.0][op.1 + 1][op.2] = 2; out.push_back((op.0,op.1 + 1,op.2))}}
    if op.2 + 1 < SIZE{if all_rocks[op.0][op.1][op.2 + 1] == 0 {all_rocks[op.0][op.1][op.2 + 1] = 2; out.push_back((op.0,op.1,op.2 + 1))}}
    if op.0 > 0{if all_rocks[op.0 - 1][op.1][op.2] == 0 {all_rocks[op.0 - 1][op.1][op.2] = 2; out.push_back((op.0 - 1,op.1,op.2))}}
    if op.1 > 0{if all_rocks[op.0][op.1 - 1][op.2] == 0 {all_rocks[op.0][op.1 - 1][op.2] = 2; out.push_back((op.0,op.1 - 1,op.2))}}
    if op.2 > 0{if all_rocks[op.0][op.1][op.2 - 1] == 0 {all_rocks[op.0][op.1][op.2 - 1] = 2; out.push_back((op.0,op.1,op.2 - 1))}}
    out
}

pub(crate) fn part_2() {
    // first, I map the actual lava
    // Then, I color each square that touches the outside air with a value of 2
    // Then, I find the surface area of all cubes with a value of 0 and subtract
    // That from the value of part 1
    let mut all_rocks = [[[0; SIZE]; SIZE]; SIZE];
    let mut surface_area = 0u32;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            let op = read_ordered_pair(&line.trim());
            all_rocks[op.0][op.1][op.2] = 1;
            surface_area += 6;
            surface_area -= count_neighbors_int(op, &all_rocks)*2;
        }
    }
    // Now we visit all the places that are outside of the rocks and turn them to rock too lol
    let mut places_to_see: VecDeque<(usize, usize, usize)> = VecDeque::new();
    places_to_see.push_back((0usize,0usize,0usize));
    places_to_see.push_back((SIZE-1,SIZE-1,SIZE-1));
    while places_to_see.len() > 0{
        let place = places_to_see.pop_front().unwrap();
        places_to_see.append(&mut color_neighbors_int(place, &mut all_rocks))
    }
    // now we build a new map from just the places where the old map is 0;
    let mut pocket_surface_area = 0u32;
    for i in 0..SIZE {
        for j in 0..SIZE {
            for k in 0..SIZE {
                if all_rocks[i][k][j] == 0 {
                    pocket_surface_area += count_neighbors_int((i, j, k), &all_rocks);
                }
            }
        }
    }

    println!("Day {DAY} Part 2: {}", surface_area-pocket_surface_area); //2624 is too high
}
