use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 22;
// problem at https://adventofcode.com/2022/day/22
// Monkey Map! Zooming through the bananosphere!

#[derive(Copy, Clone, Debug)]
enum Facing{
    Up, Down, Left, Right,
}

fn load_map() -> (Vec<Vec<char>>, String){
    let mut out: Vec<Vec<char>> = vec![vec![' ';151];200];
    // let mut out: Vec<Vec<char>> = vec![vec![' ';17];12];
    if let Ok(lines) = read_day_input_lines(DAY) {
        let mut get_instructions = false;
        for (out_idx, line) in lines.flatten().enumerate() {
            if line.trim() == ""{get_instructions = true; continue}
            if get_instructions{
                return (out, line)
            }
            for (idx, c) in line.chars().enumerate(){
                if c == '\n'{ break }
                else{
                    out[out_idx][idx] = c
                }
            }
        }
    }
    panic!("We have somehow missed the instructions!")
}

fn parse_instructions(s: &str) -> Vec<(usize, char)>{
    let turns: Vec<char> = s.matches(char::is_alphabetic).map(|s| s.chars().collect::<Vec<char>>()[0]).collect();
    let dists: Vec<usize> = s.split(char::is_alphabetic).map(|s| s.parse::<usize>().unwrap()).collect();
    let mut out: Vec<(usize, char)> = Vec::with_capacity(1203);
    for (idx, turn) in turns.into_iter().enumerate(){
        out.push((dists[idx], turn))
    }
    out.push((dists[dists.len()-1], ' '));
    out
}

/// Make maps of how far it is possible to move in a given direction
fn make_bounds(map: &[Vec<char>]) -> (Vec<(usize, usize)>,Vec<(usize, usize)>){
    let mut row_bounds = vec![(0,0);map.len()];
    let mut col_bounds = vec![(0,map.len()-1);map[0].len()];

    for (idx, r) in map.iter().enumerate(){
        row_bounds[idx].0 = r.iter().position(|&r| r != ' ').unwrap();
        row_bounds[idx].1 = r.len() - 1 - r.iter().rev().position(|&r| r != ' ').unwrap();
    }
    for col in 0..map[0].len() {
        let mut lower_unset = true;
        for row in 0..map.len() {
            if lower_unset {
                if map[row][col] != ' ' {
                    lower_unset = false;
                    col_bounds[col].0 = row
                }
            } else if map[row][col] == ' ' {
                col_bounds[col].1 = row - 1;
                break
            }
        }
    }

    (row_bounds, col_bounds)
}

fn find_start(map: &[Vec<char>], row_bounds: &[(usize, usize)]) -> usize{
    for idx in row_bounds[0].0..row_bounds[0].1+1 {
        if map[0][idx] == '.'{return idx}
    }
    panic!("Start not found!")
}

fn one_step(
    x_0: usize, y_0: usize, facing: &Facing,
    map: &[Vec<char>], row_bounds: &[(usize, usize)], col_bounds: &[(usize, usize)]
) -> (usize, usize, Facing){
    let bounds = (row_bounds[y_0], col_bounds[x_0]);
    let target_x; let target_y;
    match facing{
        Facing::Up => {
            target_y = if y_0 == bounds.1.0{bounds.1.1} else {y_0 - 1};
            target_x = x_0;
        }
        Facing::Down => {
            target_y = if y_0 == bounds.1.1{bounds.1.0} else {y_0 + 1};
            target_x = x_0;
        }
        Facing::Left => {
            target_y = y_0 ;
            target_x = if x_0 == bounds.0.0{bounds.0.1} else {x_0 - 1};
        }
        Facing::Right => {
            target_y = y_0;
            target_x = if x_0 == bounds.0.1{bounds.0.0} else {x_0 + 1};
        }
    }
    match map[target_y][target_x]{
        '#' => (x_0, y_0, *facing),
        '.' => (target_x, target_y, *facing),
        c => panic!("The character '{c}' should not be a valid target!")
    }
}

fn turn(facing: &Facing, instruction: &char) -> Facing{
    match instruction {
        ' ' => *facing,
        'L' => match facing {
            Facing::Up    => Facing::Left,
            Facing::Left  => Facing::Down,
            Facing::Down  => Facing::Right,
            Facing::Right => Facing::Up,
        },
        'R' => match facing {
            Facing::Up    => Facing::Right,
            Facing::Left  => Facing::Up,
            Facing::Down  => Facing::Left,
            Facing::Right => Facing::Down,
        },
        _ => panic!("Unexpected turn command!"),
    }
}

fn navigate_map(
    x_0: usize, y_0: usize, facing: &mut Facing, inst_vec: &[(usize, char)],
    map: &[Vec<char>], row_bounds: &[(usize, usize)], col_bounds: &[(usize, usize)],
) -> (usize, usize) {
    let mut x = x_0;
    let mut y = y_0;
    let mut f = *facing;
    for (steps, turn_dir) in inst_vec.iter(){
        // print!("{: >2}, {} => ",steps, turn_dir);
        let mut new_x;
        let mut new_y;
        for _ in 0..*steps{
            (new_x, new_y, f) = one_step(x, y, &f, map, row_bounds, col_bounds);
            if new_x == x && new_y == y {break} else {x = new_x; y = new_y;}
        }
        f = turn(&f, turn_dir);
        // print!("x: {x: >2}, y: {y: >2}, Facing: {:?}. ",f);
        // println!("that square is {}",map[y][x]);
    }
    *facing = f;
    (x, y)
}

pub(crate) fn part_1() {
    let (map, instructions) = load_map();
    let (row_bounds, col_bounds) = make_bounds(&map);
    let inst_vec = parse_instructions(&instructions);

    let mut facing: Facing = Facing::Right;
    let mut x = find_start(&map, &row_bounds);
    let mut y = 0usize;
    //println!("START! x: {x}, y: {y}, Facing: {:?}",facing);
    (x, y) = navigate_map(x, y, &mut facing, &inst_vec, &map, &row_bounds, &col_bounds);

    let ans = 1000 * (y+1) + 4 * (x+1) + match facing{
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3,
    };

    println!("\nDay {DAY} Part 1: {ans}");
}

fn cube_step(
    x_0: usize, y_0: usize, facing: &Facing,
    map: &[Vec<char>], row_bounds: &[(usize, usize)], col_bounds: &[(usize, usize)]
) -> (usize, usize, Facing){
    let (target_x, target_y, target_facing) = target_space_and_position(
        x_0, y_0, facing, row_bounds, col_bounds
    );
    match map[target_y][target_x]{
        '#' => (x_0, y_0, *facing),
        '.' => (target_x, target_y, target_facing),
        c => {
            println!("The character '{c}' at target {target_x},{target_y} should not be a valid target (from {x_0},{y_0})!");
            println!("Face of {x_0},{y_0} = {}, facing {facing:?}", get_face(x_0, y_0));
            println!("Target is {target_x},{target_y}, {target_facing:?}");
            panic!()
        }
    }
}
// Map of faces
//   0  1  2
//      ┌-┐┌-┐
// 0    |1||2|
//      └-┘└-┘
//      ┌-┐
// 1    |3|
//      └-┘
//   ┌-┐┌-┐
// 2 |4||5|
//   └-┘└-┘
//   ┌-┐
// 3 |6|
//   └-┘

fn get_face(x_0: usize, y_0: usize) -> usize{
    let box_x = x_0/50;
    let box_y = y_0/50;

    match box_x{
        0 => match box_y {
            2 => return 4,
            3 => return 6,
            _ => panic!("box_x, box_y of {box_x},{box_y} should not be possible")
        },
        1 => match box_y {
            0 => return 1,
            1 => return 3,
            2 => return 5,
            _ => panic!("box_x, box_y of {box_x},{box_y} should not be possible")
        },
        2 => if box_y == 0 {return 2},
        _ => panic!("box_x, box_y of {box_x},{box_y} should not be possible")
    }
    panic!("box_x, box_y of {box_x},{box_y} should not be possible")
}

fn target_space_and_position(
    x_0: usize, y_0: usize, facing: &Facing, row_bounds: &[(usize, usize)], col_bounds: &[(usize, usize)]
)-> (usize, usize, Facing){
    let bounds = (row_bounds[y_0], col_bounds[x_0]);
    // If we aren't going over an edge, return the original facing
    match facing{
        Facing::Up => {if y_0 != bounds.1.0{return   (x_0, y_0-1, Facing::Up)}}
        Facing::Down => {if y_0 != bounds.1.1{return (x_0, y_0+1, Facing::Down)}}
        Facing::Left => {if x_0 != bounds.0.0{return (x_0-1, y_0, Facing::Left)}}
        Facing::Right => {if x_0 != bounds.0.1{return(x_0+1, y_0, Facing::Right)}}
    }
    // We are going over an edge!
    match get_face(x_0, y_0){
        1 =>  match facing {
            Facing::Up => (0, 100 + x_0, Facing::Right), // side 6
            Facing::Left => (0, 149 - y_0, Facing::Right), // side 4
            _ => panic!("Face 1 should only be able to go Up or Left")
        }
        2 =>  match facing {
            Facing::Up => (x_0 - 100, 199, Facing::Up), // side 6
            Facing::Down => (99, x_0 - 50, Facing::Left), // side 3
            Facing::Right => (99, 149-y_0, Facing::Left), // side 5
            _ => panic!("Face 2 cannot go Left!")
        }
        3 =>  match facing {
            Facing::Right => (50 + y_0, 49, Facing::Up), // side 2
            Facing::Left  => (y_0 - 50, 100, Facing::Down), // side 4
            _ => panic!("Face 3 should only be able to go Right or Left")
        }
        4 =>  match facing {
            Facing::Up   => (50, 50 + x_0, Facing::Right), // side 3
            Facing::Left => (50, 149 - y_0, Facing::Right), // side 1
            _ => panic!("Face 4 should only be able to go Up or Left")
        }
        5 =>  match facing {
            Facing::Down  => (49, 100 + x_0, Facing::Left), // side 6
            Facing::Right => (149, 149 - y_0, Facing::Left), // side 2
            _ => panic!("Face 5 should only be able to go Down or Right")
        }
        6 =>  match facing {
            Facing::Left  => (y_0 - 100, 0, Facing::Down),
            Facing::Down  => (x_0 + 100, 0, Facing::Down),
            Facing::Right => (y_0 - 100, 149, Facing::Up),
            _ => panic!("Face 6 should only be able to go Up or Left")
        }
        wrong => panic!("{wrong} is not a cube face!")
    }
}

fn navigate_cube(
    x_0: usize, y_0: usize, facing: &mut Facing, inst_vec: &[(usize, char)],
    map: &[Vec<char>], row_bounds: &[(usize, usize)], col_bounds: &[(usize, usize)],
) -> (usize, usize) {
    let mut x = x_0;
    let mut y = y_0;
    let mut f = *facing;
    for (steps, turn_dir) in inst_vec.iter(){
        // print!("{: >2}, {} => ",steps, turn_dir);
        let mut new_x;
        let mut new_y;
        for _ in 0..*steps{
            (new_x, new_y, f) = cube_step(x, y, &f, map, row_bounds, col_bounds);
            if new_x == x && new_y == y {break} else {x = new_x; y = new_y;}
        }
        f = turn(&f, turn_dir);
        // print!("x: {x: >2}, y: {y: >2}, Facing: {:?}. ",f);
        // println!("that square is {}",map[y][x]);
    }
    *facing = f;
    (x, y)
}

pub(crate) fn part_2() {
    // The mapping is hard enough, I'm not going to try to programmatically fold the cube.
    let (map, instructions) = load_map();
    let (row_bounds, col_bounds) = make_bounds(&map);
    let inst_vec = parse_instructions(&instructions);

    let mut facing: Facing = Facing::Right;
    let mut x = find_start(&map, &row_bounds);
    let mut y = 0usize;
    //println!("START! x: {x}, y: {y}, Facing: {:?}",facing);



    (x, y) = navigate_cube(x, y, &mut facing, &inst_vec, &map, &row_bounds, &col_bounds);

    let ans = 1000 * (y+1) + 4 * (x+1) + match facing{
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3,
    };

    println!("\nDay {DAY} Part 1: {ans}");
}
