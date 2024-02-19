use std::cmp::Ordering;
use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 15;
// problem at https://adventofcode.com/2022/day/15

fn parse_line(s: &String) -> ((i64, i64, i64),(i64,i64)){
    let mut o: Vec<i64> = vec![];
    let splits = s.trim().split(&['=',',',':']).into_iter();
    for n in splits{
        o.push(match n.parse::<i64>(){
           Ok(n) => n,
            Err(_) => continue,
        })
    }
    return (
        (o[0], o[1], ((o[0] - o[2]).abs() + (o[1] - o[3]).abs()) as i64),
        (o[2], o[3])
    )
}

fn read_input() -> (Vec<(i64, i64, i64)>,Vec<(i64, i64)>){
    let mut sensors: Vec<(i64, i64, i64)> = vec![];
    let mut beacons: Vec<(i64, i64)> = vec![];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            let (s, b) = parse_line(&line);
            sensors.push(s);
            beacons.push(b);
        }
    }
    return (sensors, beacons)
}

fn calc_bounds(sensors: &Vec<(i64, i64, i64)>) -> ((i64,i64),(i64,i64)){
    let mut x_min = sensors[0].0 - sensors[0].2;
    let mut x_max = sensors[0].0 + sensors[0].2;
    let mut y_min = sensors[0].1 - sensors[0].2;
    let mut y_max = sensors[0].1 + sensors[0].2;
    for s in sensors.into_iter(){
        if s.0 - s.2 < x_min{ x_min = s.0 - s.2 }
        if s.0 + s.2 > x_max{ x_max = s.0 + s.2 }
        if s.1 - s.2 < y_min{ y_min = s.1 - s.2 }
        if s.1 + s.2 > y_max{ y_max = s.1 + s.2 }
    }
    return ((x_min, x_max),(y_min, y_max))
}

fn covered_row(sensors: &Vec<(i64, i64, i64)>, row: i64, bounds: &((i64,i64),(i64,i64))) -> Vec<bool>{
    if (row < bounds.1.0) || (row > bounds.1.1){panic!("Row {row} is outside of bounds {bounds:?}")}
    let mut out = vec![false; (1 + bounds.0.1 - bounds.0.0) as usize];
    for s in sensors.into_iter(){
        if (row < s.1 - s.2) || (row > s.1 + s.2){continue}
        let dist = match row.cmp(&s.1){
            Ordering::Equal => s.2,
            Ordering::Less => s.2 - (s.1 - row).abs(),
            Ordering::Greater => s.2 - (s.1 - row).abs(),
        };
        for idx in (s.0 - dist)..(1 + s.0 + dist) {
            out[(idx-bounds.1.0) as usize] = true;
        }
    }
    return out
}

fn string_row(sensors: &Vec<(i64, i64, i64)>, beacons: &Vec<(i64, i64)>, row: i64, bounds: &((i64, i64), (i64, i64))) -> Vec<char>{
    if (row < bounds.1.0) || (row > bounds.1.1){panic!("Row {row} is outside of bounds {bounds:?}")}
    let mut out = vec!['.'; (1 + bounds.0.1 - bounds.0.0) as usize];

    for b in beacons.into_iter(){
        if b.1 == row{ out[(b.0-bounds.1.0) as usize] = 'B'}
    }

    for s in sensors.into_iter(){
        if s.1 == row{ out[(s.0-bounds.1.0) as usize] = 'S'}
        if (row < s.1 - s.2) || (row > s.1 + s.2){continue}
        let dist = match row.cmp(&s.1){
            Ordering::Equal => s.2,
            Ordering::Less => s.2 - (s.1 - row).abs(),
            Ordering::Greater => s.2 - (s.1 - row).abs(),
        };
        for idx in (s.0 - dist)..(1 + s.0 + dist) {
            if out[(idx-bounds.1.0) as usize] == '.'{
                out[(idx-bounds.1.0) as usize] = '#';
        }}
    }
    return out
}


pub(crate) fn part_1() {
    let (sensors, beacons) = read_input();
    let bounds = calc_bounds(&sensors);
    let mut visible_count = 0;
    // let r = string_row(&sensors, &beacons, 10,&bounds);
    let r = string_row(&sensors, &beacons, 2000000,&bounds);
    for c in r.into_iter(){
        if {(c!='.') & (c!='B')}{visible_count += 1}
    }
    println!("Day {DAY} Part 1: {visible_count}");
}

pub(crate) fn part_2() {
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
        }
    }
    println!("Day {DAY} Part 2: incomplete");
}
