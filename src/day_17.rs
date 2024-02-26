use std::cmp::{max};

use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 17;
// problem at https://adventofcode.com/2022/day/17

#[derive(Clone)]
struct Rock{
    height_of_bottom: usize,
    layers: Vec<u8>,
}

fn generate_wide(top_of_rocks: usize) -> Rock{
    Rock{
        height_of_bottom: top_of_rocks + 3,
        layers: vec![0b0001_1110u8],
    }
}

fn generate_plus(top_of_rocks: usize) -> Rock{
    Rock{
        height_of_bottom: top_of_rocks + 3,
        layers: vec![0b0000_1000u8,
                     0b0001_1100u8,
                     0b0000_1000u8],
    }
}

fn generate_L(top_of_rocks: usize) -> Rock{
    Rock{
        height_of_bottom: top_of_rocks + 3,
        layers: vec![0b0001_1100u8,
                     0b0000_0100u8,
                     0b0000_0100u8],
    }
}

fn generate_l(top_of_rocks: usize) -> Rock{
    Rock{
        height_of_bottom: top_of_rocks + 3,
        layers: vec![0b0001_0000u8,
                     0b0001_0000u8,
                     0b0001_0000u8,
                     0b0001_0000u8],
    }
}

fn generate_square(top_of_rocks: usize) -> Rock{
    Rock{
        height_of_bottom: top_of_rocks + 3,
        layers: vec![0b0001_1000u8,
                     0b0001_1000u8],
    }
}

fn generate_rock(top_of_rocks: usize, rock_count: usize) -> Rock{
    match rock_count % 5{
        0 => generate_wide(top_of_rocks),
        1 => generate_plus(top_of_rocks),
        2 => generate_L(top_of_rocks),
        3 => generate_l(top_of_rocks),
        4 => generate_square(top_of_rocks),
        _ => panic!("This should not be possible")
    }
}

fn slide_left(rock: &Rock, chamber: &[u8]) -> Rock{
    let new_rock_layers = rock.layers.clone().into_iter().map(|r| r<<1).collect::<Vec<u8>>();
    for (idx, layer) in new_rock_layers.iter().enumerate(){
        if (chamber[rock.height_of_bottom + idx] & layer > 0) || (layer >= &0b1000_0000u8){
            return rock.clone()
        }
    }
    Rock{
        height_of_bottom: rock.height_of_bottom,
        layers: new_rock_layers,
    }
}

fn slide_right(rock: &Rock, chamber: &[u8]) -> Rock{
    for layer in rock.layers.iter(){
        if layer % 2 == 1{return rock.clone()}
    }
    let new_rock_layers = rock.layers.clone().into_iter().map(|r| r>>1).collect::<Vec<u8>>();
    for (idx, layer) in new_rock_layers.iter().enumerate(){
        if chamber[rock.height_of_bottom + idx] & layer > 0{
            return rock.clone()
        }
    }
    Rock{
        height_of_bottom: rock.height_of_bottom,
        layers: new_rock_layers,
    }
}

fn slide_rock(rock: &Rock, slide: char, chamber: &[u8]) -> Rock{
    match slide{
    '<' => slide_left(rock, chamber),
    '>' => slide_right(rock, chamber),
    _ => panic!("That's not a direction, friend")
    }
}

fn drop_rock(rock: &Rock, chamber: &mut [u8]) -> Option<Rock>{
    if rock.height_of_bottom == 0{
        fix_rock(rock, chamber);
        return None
    }
    let new_rock = Rock{
        height_of_bottom: rock.height_of_bottom - 1,
        layers: rock.layers.clone()
    };
    for (idx, layer) in new_rock.layers.iter().enumerate(){
        if chamber[new_rock.height_of_bottom + idx] & layer > 0{
            fix_rock(rock, chamber);
            return None
        }
    }
    Some(new_rock)
}

fn fix_rock(rock: &Rock, chamber: &mut [u8]){
    for (idx, layer) in rock.layers.iter().enumerate(){
        if chamber[rock.height_of_bottom + idx] & layer > 0{
            panic!("There should be no overlap");
        }
        chamber[rock.height_of_bottom + idx] += layer;
    }
}

fn print_board(chamber: &[u8], height: usize){
    for row in (0..height+1).rev(){
        let s = str::replace(&format!("|{:07b}|", chamber[row]),"0",".");
        println!("{}",str::replace(&s,"1","#"));
    }
    println!("|-------|");
}

fn print_full_row_numbers(chamber: &[u8], height: usize){
    for row in (0..height+1).rev(){
        if chamber[row] == 0b0111_1111u8 {println!("{row:>3}")}
            }
}

pub(crate) fn part_1() {
    // The Vec<Vec<bool>> style takes 97.91 ms for 20,000 rocks and is >O(n)
    // The Vec<u8> version takes 51.62 ms for 20,000 rocks and is closer to O(n)
    // The Vec<u8> version takes 640 ms for 200,000 rocks.
    // That's 34 days for 1000000000000 rocks... there has to be some kind of repeating pattern
    // cargo run --release takes 61 ms, but still 3.4 days for 1 trillion rocks
    let mut chamber: Vec<u8> = vec![0u8;310000];
    let mut top_of_rocks = 0usize;
    let commands;
    let mut command_idx = 0usize;
    if let Ok(lines) = read_day_input_lines(DAY) {
        if let Some(line) = lines.flatten().next() {
            commands = line.trim().chars().collect::<Vec<char>>();
        } else {panic!("No inputs in file!")}
    } else {panic!("Some kind of error with reading the file")}
    let n_commands = commands.len();
    println!("n_commands: {n_commands}");

    use std::time::Instant;
    let now = Instant::now();
    for rock_idx in 0..2022{
        let mut rock = generate_rock(top_of_rocks, rock_idx);
        loop {
            rock = slide_rock(&rock, commands[command_idx], &chamber);
            command_idx = (command_idx + 1) % n_commands;
            match drop_rock(&rock, &mut chamber){
                Some(r) => rock = r,
                None => {top_of_rocks = max(rock.layers.len() + rock.height_of_bottom, top_of_rocks); break},
                };
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    //print_board(&chamber, top_of_rocks);
    println!("Day {DAY} Part 1: {}",top_of_rocks);
}

fn get_full_row_idx(chamber: &[u8], height: usize)-> Vec<u8>{
    let mut idxs = vec![];
    for (_, row) in chamber.iter().enumerate().take(height+1){
        if *row == 0b0111_1111u8 {
            idxs.push(*row)
        }
    }
    idxs
}

pub(crate) fn part_2() {
    // Now the same thing, but one trillion blocks! There must be some kind of repeating pattern
    // or it wouldn't be possible.
    // for my input, there is a pattern of 2666 rows, about 1750 blocks. It starts between rows
    // 403 and 778 (which repeats at 3268).
    let mut chamber: Vec<u8> = vec![0u8;40000];
    let mut top_of_rocks = 0usize;
    let commands;
    let mut command_idx = 0usize;
    if let Ok(lines) = read_day_input_lines(DAY) {
        if let Some(line) = lines.flatten().next() {
            commands = line.trim().chars().collect::<Vec<char>>();
        } else {panic!("No inputs in file!")}
    } else {panic!("Some kind of error with reading the file")}
    let n_commands = commands.len();
    println!("n_commands: {n_commands}");


    let check_depth = 50;
    // rock_idx, command_idx, top_of_rocks, chamber from top_of_rocks 0 to top_of_rocks -10
    let mut proposed_repeat = (0usize, 0usize, 0, vec![0u8;check_depth]);
    let total_rocks = 1000000000000usize;
    let mut n_repeats = 0usize;
    let mut height_boost = 0usize;
    let mut rock_idx = 0usize;
    'outer: while rock_idx < total_rocks{
        let mut rock = generate_rock(top_of_rocks, rock_idx);
        loop {
            // Get the repeat pattern. Once the repeat has been found, blast rock_idx to stratosphere
            if (top_of_rocks >= 800) & (proposed_repeat.0 == 0){
                proposed_repeat = (
                    rock_idx,
                    command_idx,
                    top_of_rocks,
                    (chamber[top_of_rocks-check_depth..top_of_rocks+1]).to_vec()
                )
            } else if (n_repeats==0) & (top_of_rocks >= 3268) & (rock_idx % 5 == proposed_repeat.0%5) & (command_idx == proposed_repeat.1) {
                let mut correct_count = 0;
                for row in 0..check_depth+1{
                    if chamber[top_of_rocks-check_depth + row] == proposed_repeat.3[row]{
                        correct_count += 1;
                    }
                    if correct_count == check_depth+1{
                        println!("Found a match! {}, {}, {}", proposed_repeat.0, proposed_repeat.1, proposed_repeat.2);
                        println!("repeats at rock_idx: {rock_idx}, top_of_rocks {top_of_rocks}");
                        let repeat_n_rocks = rock_idx - proposed_repeat.0;
                        let repeat_n_rows = top_of_rocks - proposed_repeat.2;
                        n_repeats = (total_rocks - rock_idx)/repeat_n_rocks;
                        height_boost = repeat_n_rows * n_repeats;
                        rock_idx += n_repeats * repeat_n_rocks
                    }
                }
            }
            rock = slide_rock(&rock, commands[command_idx], &chamber);
            command_idx = (command_idx + 1) % n_commands;
            match drop_rock(&rock, &mut chamber){
                Some(r) => rock = r,
                None => {
                    top_of_rocks = max(rock.layers.len() + rock.height_of_bottom, top_of_rocks);
                    rock_idx += 1;
                    break
                },
            };
        }
    }
    // This helped me find the repeat:
    // let full_row_idxs = get_full_row_idx(&chamber, top_of_rocks);
    // let full_row_dists: Vec<usize> = full_row_idxs.windows(2)
    //     .map(|slice| slice[1] - slice[0]).collect();
    //
    // println!("{:?}",full_row_dists); // every 16 full rows, the pattern repeats
    // let ps: usize = 2;
    // println!("From row {} (diff {}) to row {} (diff {}) are {} rows apart",
    //          full_row_idxs[ps],full_row_dists[ps],full_row_idxs[ps+16],full_row_dists[ps+16],
    //      full_row_idxs[ps+16] - full_row_idxs[ps]
    // );
    println!("Day {DAY} Part 2: {}", height_boost + top_of_rocks);
}