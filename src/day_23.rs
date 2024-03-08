use std::collections::HashMap;
use std::iter::Map;
use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 23;
// problem at https://adventofcode.com/2022/day/23
// The Elf Game of Planting!

#[derive(Clone, Debug, Hash, PartialOrd, Eq, PartialEq)]
struct Location {x:usize, y:usize}

const ORDER: &str = "NSEWNSE ";
const MAP_SIZE: usize = 300;

fn load_map() -> Vec<Vec<bool>>{
    let mut out: Vec<Vec<bool>> = vec![vec![false;MAP_SIZE];MAP_SIZE];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for (out_idx, line) in lines.flatten().enumerate() {
            if line.trim() == ""{break}
            for (idx, c) in line.chars().enumerate(){
                if c == '\n'{ break }
                else if c == '#'{
                    out[out_idx+MAP_SIZE/3][idx+MAP_SIZE/3] = true
                }
            }
        }
    }
    return out
}

fn propose_destination(loc: &Location, round:usize, map: &[Vec<bool>])-> Location{
    // check to see if elf is alone
    let mut alone = true;
    'x: for dx in -1..=1{
        for dy in -1..=1{
            if dx == 0 && dy == 0 {continue}
            if map[(loc.y as i32 + dy) as usize][(loc.x as i32 + dx) as usize]{alone=false; break 'x}
        }
    }
    if alone{return loc.clone()}

    // figure out which way to move!
    for d in ORDER.chars().skip(round%4).take(4){
        match d{
            'N' => if map[loc.y-1][loc.x] || map[loc.y-1][loc.x-1] || map[loc.y-1][loc.x+1] {
                continue
            } else { return Location { y: loc.y - 1, .. *loc }; },

            'S' => if map[loc.y+1][loc.x] || map[loc.y+1][loc.x-1] || map[loc.y+1][loc.x+1] {
                continue
            } else { return Location { y: loc.y + 1, .. *loc }; },

            'E' => if map[loc.y-1][loc.x-1] || map[loc.y][loc.x-1] || map[loc.y+1][loc.x-1] {
                continue
            } else { return Location { x: loc.x - 1, .. *loc }; },

            'W' => if map[loc.y-1][loc.x+1] || map[loc.y][loc.x+1] || map[loc.y+1][loc.x+1] {
                continue
            } else { return Location { x: loc.x + 1, .. *loc }; },
            _ => panic!("that's just silly friend"),
        }
    }
    return loc.clone()
}

fn find_all_elves(map: &[Vec<bool>]) -> Vec<Location>{
    let mut out: Vec<Location> = Vec::with_capacity(75*75);
    for y in 0..MAP_SIZE{
        for x in 0..MAP_SIZE{
            if map[y][x] {out.push(Location{x,y})}
        }
    }
    out
}

fn get_all_proposals(all_elves: &[Location], round: usize, map: &[Vec<bool>]) -> Vec<Location>{
    let mut out = Vec::with_capacity(all_elves.len());
    for elf in all_elves{
        out.push(propose_destination(elf, round, map))
    }
    out
}

fn move_all_elves(all_elves: &[Location], destinations: &[Location], map: &[Vec<bool>])->Vec<Vec<bool>>{
    let mut new_map = vec![vec![false;MAP_SIZE];MAP_SIZE];
    let mut dest_counts: HashMap<Location, u8>= HashMap::with_capacity(all_elves.len());
    for d in destinations{
        *dest_counts.entry(d.clone()).or_insert(0) += 1;
    }

    for (d, elf) in destinations.iter().zip(all_elves.iter()){
        if dest_counts[&d] <= 1{
            new_map[d.y][d.x] = true
        } else {
            new_map[elf.y][elf.x] = true
        }
    }
    new_map
}
fn get_bounds(map: &[Vec<bool>])-> (Location, Location){
    let mut top = 0usize;
    let mut bot = 0usize;
    let mut lft = 0usize;
    let mut rgt = 0usize;
    for (y, v) in map.iter().enumerate(){
        if v.iter().any(|b| *b){
            top=y; break;
        }
    }
    for (y, v) in map.iter().rev().enumerate(){
        if v.iter().any(|b| *b){
            bot=MAP_SIZE-y-1; break;
        }
    }
    'x: for xdx in 0..MAP_SIZE{
        for ydx in 0..MAP_SIZE{
            if map[ydx][xdx]{lft = xdx; break 'x}
        }
    }
    'x: for xdx in 0..MAP_SIZE{
        for ydx in 0..MAP_SIZE{
            if map[ydx][MAP_SIZE - xdx-1]{rgt = MAP_SIZE-xdx-1; break 'x}
        }
    }

    //println!("b: {bot}  t: {top} r: {rgt}  l:{lft}");
    return (Location{x:lft,y:top}, Location{x:rgt,y:bot})
}
fn find_emptygound_in_rec(map: &[Vec<bool>], elf_count: usize)->usize{
    let (tl, br) = get_bounds(map);
    return (br.y - tl.y + 1)*(br.x - tl.x + 1) - elf_count
}

fn print_map(map: &[Vec<bool>]){
    let (tl, br) = get_bounds(map);

    for y in tl.y..=br.y{
        for x in tl.x..=br.x{
            if y==6 + MAP_SIZE/3 && x == 2 + MAP_SIZE/3 {print!("@")}
            else if map[y][x] {print!("#")} else {print!(".")}
        }
        print!("\n")
    }
}

pub(crate) fn part_1() {
    let mut map = load_map();
    let mut elves= find_all_elves(&map);
    let mut dests;
    for round in 0..10{
        elves = find_all_elves(&map);
        dests = get_all_proposals(&elves, round, &map);
        map = move_all_elves(&elves, &dests, &map);
    }
    let ans = find_emptygound_in_rec(&map, elves.len());
    println!("Day {DAY} Part 1: {ans}");
}

pub(crate) fn part_2() {
    let mut map = load_map();
    let mut new_map = map.clone();
    let mut elves= find_all_elves(&map);
    let mut dests;
    let mut round = 0usize;
    loop{
        elves = find_all_elves(&map);
        dests = get_all_proposals(&elves, round, &map);
        round += 1;
        if elves == dests{break}
        map = move_all_elves(&elves, &dests, &map);
    }
    println!("Day {DAY} Part 1: {round}");
}
