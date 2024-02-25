use std::cmp::{min, max};
use std::collections::VecDeque;
use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 17;
// problem at https://adventofcode.com/2022/day/17

fn generate_wide(top_of_rocks: usize) -> Vec<(usize, usize)>{
    let x = top_of_rocks+4;
    return vec![(x, 2), (x, 3), (x, 4), (x, 5)]
}

fn generate_plus(top_of_rocks: usize) -> Vec<(usize, usize)>{
    let x = top_of_rocks+4;
    return vec![(x, 3), (x+1, 2), (x+1, 3), (x+1, 4), (x+2, 3)]
}

fn generate_L(top_of_rocks: usize) -> Vec<(usize, usize)>{
    let x = top_of_rocks+4;
    return vec![(x, 2), (x, 3), (x, 4), (x+1, 4), (x+2, 4)]
}

fn generate_l(top_of_rocks: usize) -> Vec<(usize, usize)>{
    let x = top_of_rocks+4;
    return vec![(x, 2), (x+1, 2), (x+2, 2), (x+3, 2)]
}

fn generate_square(top_of_rocks: usize) -> Vec<(usize, usize)>{
    let x = top_of_rocks+4;
    return vec![(x, 2), (x, 3), (x+1, 2), (x+1, 3)]
}

fn generate_rock(top_of_rocks: usize, rock_count: usize) -> Vec<(usize, usize)>{
    return match rock_count % 5{
        0 => generate_wide(top_of_rocks),
        1 => generate_plus(top_of_rocks),
        2 => generate_L(top_of_rocks),
        3 => generate_l(top_of_rocks),
        4 => generate_square(top_of_rocks),
        _ => panic!("This should not be possible")
    }
}

fn slide_left(rock: &[(usize, usize)], chamber: &[Vec<bool>]) -> Vec<(usize, usize)>{
    let mut new_rock = vec![];
    for (x, y) in rock.into_iter(){
        if chamber[*x][*y-1]{
            return rock.to_vec().clone()
        }
        new_rock.push((*x,*y-1));
    }
    return new_rock
}

fn slide_right(rock: &[(usize, usize)], chamber: &[Vec<bool>]) -> Vec<(usize, usize)>{
    let mut new_rock = vec![];
    for (x, y) in rock.into_iter(){
        if chamber[*x][*y+1]{
            return rock.to_vec().clone()
        }
        new_rock.push((*x,*y+1));
    }
    return new_rock
}

fn slide_rock(rock: &[(usize, usize)], slide: char, chamber: &[Vec<bool>]) -> Vec<(usize,usize)>{
    match slide{
    '<' =>{
        if min(rock[0].1, rock[1].1) == 0{
            return rock.to_vec()
        } else {
            return slide_left(rock, chamber)
        }
    },
    '>' =>{
        if rock[3].1 == 6{
            return rock.to_vec()
        } else {
            return slide_right(rock, chamber)
        }
    },
    _ => panic!("That's not a direction, friend")
    }
}

fn drop_rock(rock: &[(usize, usize)], chamber: &mut [Vec<bool>]) -> Option<Vec<(usize,usize)>>{
    let mut new_rock = vec![];
    for (x, y) in rock.into_iter(){
        if chamber[*x-1][*y]{
            fix_rock(rock, chamber);
            return None
        }
        new_rock.push((x-1,*y));
    }
    return Some(new_rock)
}

fn fix_rock(rock: &[(usize, usize)], chamber: &mut [Vec<bool>]){
    for (x, y) in rock.into_iter(){
        if chamber[*x][*y]{println!("{rock:?}"); panic!("This rock already overlaps with another rock!")}
        chamber[*x][*y] = true;
    }
}

fn print_board(chamber: &[Vec<bool>], height: usize){
    for row in (0..height+2).rev(){
        println!("{row:>3} |{}|", chamber[row].clone().into_iter()
            .map(|a| if a {'#'} else {'.'}).collect::<String>()
        );
    }
}

fn print_full_row_numbers(chamber: &[Vec<bool>], height: usize){
    for row in (0..height+2).rev(){
        if chamber[row].clone().into_iter().map(|a| if a{1}else{0}).sum::<u32>() == 7{println!("{row:>3}")}
            }
}

pub(crate) fn part_1() {
    let mut chamber:Vec<Vec<bool>> = vec![vec![false;7];5257];
    for i in 0..7{
        chamber[0][i] = true;
    }
    let mut top_of_rocks = 0usize;
    let mut commands = vec!['a'];
    let mut command_idx = 0usize;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            commands = line.trim().chars().collect::<Vec<char>>();
            break
        }
    } else {panic!("Some kind of error with reading the file")}
    let n_commands = commands.len();
    println!("n_commands: {n_commands}");
    for rock_idx in 0..2022{
        let mut rock = generate_rock(top_of_rocks, rock_idx);
        loop {
        rock = slide_rock(&rock, commands[command_idx], &chamber);
        command_idx = (command_idx + 1) % n_commands;
        match drop_rock(&rock, &mut chamber){
            Some(r) => rock = r,
            None => {top_of_rocks = max(rock[rock.len()-1].0, top_of_rocks); break},
            };
        }
    }
    print_full_row_numbers(&chamber, top_of_rocks);
    println!("Day {DAY} Part 1: {top_of_rocks}");
}

fn check_if_truncatable(chamber: &[Vec<bool>], rock: &[(usize, usize)])->bool{

    for row in rock.into_iter().map(|r| r.1){

    }

    return false
}

fn truncate_chamber(chamber: &mut VecDeque<Vec<bool>>, rock){

}

pub(crate) fn part_2() {
    // Now the same thing, but one trillion blocks!
    // I am going to start with the same strategy, only with truncating the tower.
    // I'm going to see if truncating only on full rows is enough
    let mut chamber: VecDeque<Vec<bool>>= VecDeque::from(vec![vec![false;7];5257]);
    // I should make that a VecDeque of u8 instead! Wow!
    for i in 0..7{
        chamber[0][i] = true;
    }
    let mut top_of_rocks = 0usize;
    let mut commands = vec!['a'];
    let mut command_idx = 0usize;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            commands = line.trim().chars().collect::<Vec<char>>();
            break
        }
    } else {panic!("Some kind of error with reading the file")}
    let n_commands = commands.len();
    println!("n_commands: {n_commands}");
//     for rock_idx in 0..2022{
//         let mut rock = generate_rock(top_of_rocks, rock_idx);
//         loop {
//         rock = slide_rock(&rock, commands[command_idx], &chamber);
//         command_idx = (command_idx + 1) % n_commands;
//         match drop_rock(&rock, &mut chamber){
//             Some(r) => rock = r,
//             None => {top_of_rocks = max(rock[rock.len()-1].0, top_of_rocks); break},
//             };
//         }
//
//     }
    println!("Day {DAY} Part 2: incomplete");
}
