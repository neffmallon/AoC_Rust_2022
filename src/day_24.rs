use std::collections::VecDeque;
use std::fmt;
use num::integer::lcm;
use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 24;
const WALL:u32 = 4294967295;
// problem at https://adventofcode.com/2022/day/24
// navigating the blizzard!

/// left and right: the rows all need to cycle, so each row is a VecDeque
/// up and down: the *columns* need to cycle, so the outer structure should be a VecDeque
#[derive(Clone, Debug, Eq, PartialEq)]
struct Bliz{
    up: VecDeque<Vec<bool>>,
    down: VecDeque<Vec<bool>>,
    left: Vec<VecDeque<bool>>,
    right: Vec<VecDeque<bool>>,
}
impl fmt::Display for Bliz{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write first row:
        write!(f, "#.")?;
        for _ in 0..self.up[0].len(){write!(f, "#")?}
        write!(f, "\n")?;
        for row in 0..self.up.len(){
        write!(f, "#")?;
            for col in 0..self.up[0].len(){
                let c = self.up[row][col] as u8
                    + self.down[row][col] as u8
                    + self.left[row][col] as u8
                    + self.right[row][col] as u8;
                if c == 0{write!(f, ".")?; continue}
                if c != 1{write!(f,"{}", c)?; continue}
                else if self.up[row][col]{write!(f, "^")?}
                else if self.down[row][col]{write!(f, "v")?}
                else if self.left[row][col]{write!(f, "<")?}
                else if self.right[row][col]{write!(f, ">")?}
                else{panic!("I have somehow made a programming error.")}
            }
            write!(f, "#\n")?
        }
        for _ in 0..self.up[0].len(){write!(f, "#")?}
        write!(f, ".#")?;
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
struct Location{row:usize, col:usize}

fn get_dimensions()->(usize, usize, usize){
    let mut rows = 0;
    let mut cols = 0;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if cols == 0{cols = line.len()-2}
            if line.trim()==""{break}
            rows+=1
        }
    }
    (rows-2, cols, lcm(rows-2, cols))
}


fn read_input()->Bliz{
    let (rows, cols, _) = get_dimensions();
    let mut left:Vec<VecDeque<bool>> = vec![VecDeque::from(vec![false;cols]);rows];
    let mut right:Vec<VecDeque<bool>> = vec![VecDeque::from(vec![false;cols]);rows];
    let mut up:VecDeque<Vec<bool>> = VecDeque::from(vec![vec![false;cols];rows]);
    let mut down:VecDeque<Vec<bool>> = VecDeque::from(vec![vec![false;cols];rows]);

    if let Ok(lines) = read_day_input_lines(DAY) {
        for (row, s) in lines.map_while(Result::ok).skip(1).enumerate() {
            for (col, c) in s.chars().skip(1).enumerate(){
                match c{
                    '<' => left[row][col] = true,
                    '>' => right[row][col] = true,
                    '^' => up[row][col] = true,
                    'v' => down[row][col] = true,
                    _ => continue
                }
            }
        }
    }

    Bliz{up, down, right, left}
}

fn rotate_bliz(bliz: &mut Bliz, n: usize){
    let row_n = n % bliz.left[0].len();
    let col_n = n % bliz.left.len();
    for row in bliz.left.iter_mut(){
        row.rotate_left(row_n)
    }
    for row in bliz.right.iter_mut(){
        row.rotate_right(row_n)
    }
    bliz.up.rotate_left(col_n);
    bliz.down.rotate_right(col_n);
}

fn destinations(l: &Location, m_r: usize, m_c:usize)->Vec<Location>{
    let mut out= vec![l.clone()];
    if l.row > 0{ out.push(Location{row: l.row - 1, col:l.col})}
    if l.col > 0{ out.push(Location{col: l.col - 1, row:l.row})}
    if l.row < m_r{ out.push(Location{row: l.row + 1, col:l.col})}
    if l.col < m_c{ out.push(Location{col: l.col + 1, row:l.row})}
    out
}

fn is_not_clear(bliz: &Bliz, r: usize, c: usize)->bool{
    bliz.up[r][c] || bliz.down[r][c] || bliz.right[r][c] || bliz.left[r][c]
}

fn run_checks(
    current_turn: u32,
    iter_slice: &[Location],
    mut_vec: &mut Vec<Location>,
    bliz: &Bliz,
    time_map:&mut [Vec<Vec<u32>>],
    max_loc: &Location,
    end_loc: &Location,
) -> bool{
    let time_idx = ((current_turn-1) % time_map.len() as u32) as usize;
    for loc in iter_slice.iter() {
            let ds = destinations(loc, max_loc.row, max_loc.col);
            for d in ds.iter(){
                // if the destination is so far empty
                if time_map[time_idx][d.row][d.col] == 0 {
                    // check to make sure there is no snow
                    if is_not_clear(bliz, d.row, d.col) {
                        time_map[time_idx][d.row][d.col] = WALL
                    } else {  // If we are snow free, mark the spot with the turn number
                        time_map[time_idx][d.row][d.col] = current_turn;
                        mut_vec.push(d.clone());
                        if d == end_loc {return true}
                    }
                }
            }
        }
    return false
}


fn get_length_min_path(bliz: &mut Bliz, forward: bool)->u32{
    let (rows, cols, duration) = get_dimensions();
    let max_loc = Location{row:rows-1, col:cols-1};
    let end = if forward{max_loc.clone()} else {Location{row:0, col:0}};
    let start = if !forward{max_loc.clone()} else {Location{row:0, col:0}};
    let mut time_map: Vec<Vec<Vec<u32>>> = vec![vec![vec![0;cols];rows];duration];
    let mut current_turn = 1u32;
    let mut even_check = vec![];
    let mut odd_check = vec![];
    'outer: loop {
        // move all the snow!
        rotate_bliz(bliz, 1);
        let time_idx = ((current_turn-1) % duration as u32) as usize;
        // check to see if the starting square is open. If so, start another path there
        if time_map[time_idx][start.row][start.col] == 0{
            if is_not_clear(bliz, start.row, start.col){
                time_map[time_idx][start.row][start.col] = WALL;
            } else {
                time_map[time_idx][start.row][start.col] = current_turn;
                if current_turn%2 == 0{
                    odd_check.push(start.clone())
                } else {even_check.push(start.clone())}
            }
        };
        // check each of the destinations from last time to see if there is anywhere to move to
        let pls_brk;
        if current_turn%2 == 0{
            pls_brk = run_checks(
                current_turn,
                &even_check,
                &mut odd_check,
                bliz,
                &mut time_map,
                &max_loc,
                &end,
            );
            even_check.clear()
        } else {
            pls_brk = run_checks(
                current_turn,
                &odd_check,
                &mut even_check,
                bliz,
                &mut time_map,
                &max_loc,
                &end,
            );
            odd_check.clear();
        };
        if pls_brk{break 'outer}
        current_turn += 1
    }
    rotate_bliz(bliz, 1);
    return current_turn+1
}

pub(crate) fn part_1() {
    let mut bliz = read_input();
    println!("Day {DAY} Part 1: {}", get_length_min_path(&mut bliz, true)); //322
}

pub(crate) fn part_2() {
    let mut bliz = read_input();
    let mut total_minutes = get_length_min_path(&mut bliz, true);
    total_minutes += get_length_min_path(&mut bliz, false);
    total_minutes += get_length_min_path(&mut bliz, true);
    println!("Day {DAY} Part 2: {total_minutes}");
}
