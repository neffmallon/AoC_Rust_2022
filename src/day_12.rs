use std::ops::Index;
use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 12;
// problem at https://adventofcode.com/2022/day/12

fn char_to_elev(c: char) -> u8{
    match c{
        'S' => 1,
        'E' => 26,
        c => (1 + c as u32 - 'a' as u32) as u8
    }
}

fn get_peak_locs(i:usize, j:usize, bounds:((usize, usize), (usize, usize))) -> Vec<(usize, usize)>{
    let mut out = vec![];
    if i > bounds.0.0{ out.push((i-1, j))}
    if i+1 < bounds.0.1{ out.push((i+1, j))}
    if j > bounds.1.0{ out.push((i, j-1))}
    if j+1 < bounds.1.1{ out.push((i, j+1))}
    out
}

pub(crate) fn part_1() {
    let mut map: Vec<String> = vec![];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            map.push(line);
        }
    }
    // Convert elevations to numbers
    let mut elevations: Vec<Vec<u8>> = vec![];
    for l in map.iter(){
        elevations.push(l.chars().map(char_to_elev).collect());
    }
    // convert starting place to distance
    let mut distance = vec![vec![-1i32;map[0].len()];map.len()];
    let mut active_searchers: Vec<(usize, usize)> = Vec::new();
    let mut end_loc = (0usize,0usize);
    for (idx, l) in map.iter().enumerate(){
        if l.contains("S"){
            distance[idx][l.find("S").unwrap()] = 0;
            active_searchers.push((idx as usize, l.find("S").unwrap()))
        }
        if l.contains("E"){
            end_loc = (idx,l.find("E").unwrap());
        }
    }
    // set bounds
    let bounds = ((0usize, map.len()),(0usize,map[0].len()));

    // Fill out the distance matrix!
    let max_search_count = 10000;
    let mut step = 0;
    while (step < max_search_count) & (distance[end_loc.0][end_loc.1] == -1){
        step += 1;
        let step_active_searchers = active_searchers.clone();
        active_searchers = vec![];
        for s in step_active_searchers{
            let peak_locs = get_peak_locs(s.0, s.1, bounds);
            for p in peak_locs{
                if (distance[p.0][p.1] == -1) &
                    (elevations[s.0][s.1] + 1 >=  elevations[p.0][p.1]){
                    distance[p.0][p.1] = step;
                    active_searchers.push(p);
                }
            }
        }
    }
    println!("Day {DAY} Part 1: {}", distance[end_loc.0][end_loc.1]);
}

pub(crate) fn part_2() {
    let mut map: Vec<String> = vec![];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            map.push(line);
        }
    }
    // Convert elevations to numbers
    let mut elevations: Vec<Vec<u8>> = vec![];
    for l in map.iter(){
        elevations.push(l.chars().map(char_to_elev).collect());
    }
    // convert starting place to distance
    let mut distance = vec![vec![-1i32;map[0].len()];map.len()];
    let mut active_searchers: Vec<(usize, usize)> = Vec::new();
    let mut end_loc = (0usize,0usize);
    for (idx, l) in map.iter().enumerate(){
        if l.contains("E"){
            end_loc = (idx,l.find("E").unwrap());
            distance[idx][l.find("E").unwrap()] = 0;
            active_searchers.push((idx as usize, l.find("E").unwrap()))
        }
    }
    // set bounds
    let bounds = ((0usize, map.len()),(0usize,map[0].len()));

    // Fill out the distance matrix, starting from the End instead!
    let max_search_count = 1000;
    let mut step = 0;
    let mut keep_looking = true;
    while (step < max_search_count) & keep_looking {
        step += 1;
        let step_active_searchers = active_searchers.clone();
        active_searchers = vec![];
        for s in step_active_searchers{
            let peak_locs = get_peak_locs(s.0, s.1, bounds);
            for p in peak_locs{
                if (distance[p.0][p.1] == -1) &
                    (elevations[s.0][s.1] <= 1 + elevations[p.0][p.1]){
                    distance[p.0][p.1] = step;
                    active_searchers.push(p);
                    if elevations[p.0][p.1] == 1 {
                        keep_looking = false
                    }
                }
            }
        }
    }
    println!("Day {DAY} Part 2: {}", step);
}