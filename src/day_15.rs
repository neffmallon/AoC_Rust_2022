use std::cmp::{max};
use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 15;
// problem at https://adventofcode.com/2022/day/15

fn parse_line(s: &str) -> ((i64, i64, i64),(i64,i64)){
    let mut o: Vec<i64> = vec![];
    let splits = s.trim().split(&['=',',',':']);
    for n in splits{
        o.push(match n.parse::<i64>(){
           Ok(n) => n,
            Err(_) => continue,
        })
    }
    (
        (o[0], o[1], ((o[0] - o[2]).abs() + (o[1] - o[3]).abs())),
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
    (sensors, beacons)
}

fn calc_bounds(sensors: &[(i64, i64, i64)]) -> ((i64,i64),(i64,i64)){
    let mut x_min = sensors[0].0 - sensors[0].2;
    let mut x_max = sensors[0].0 + sensors[0].2;
    let mut y_min = sensors[0].1 - sensors[0].2;
    let mut y_max = sensors[0].1 + sensors[0].2;
    for s in sensors.iter(){
        if s.0 - s.2 < x_min{ x_min = s.0 - s.2 }
        if s.0 + s.2 > x_max{ x_max = s.0 + s.2 }
        if s.1 - s.2 < y_min{ y_min = s.1 - s.2 }
        if s.1 + s.2 > y_max{ y_max = s.1 + s.2 }
    }
    ((x_min, x_max),(y_min, y_max))
}


fn string_row(sensors: &[(i64, i64, i64)], beacons: &[(i64, i64)], row: i64, bounds: &((i64, i64), (i64, i64))) -> Vec<char>{
    let mut out = vec!['.'; (1 + bounds.0.1 - bounds.0.0) as usize];
    if (row < bounds.1.0) || (row > bounds.1.1){return out}

    for b in beacons.iter(){
        if b.1 == row{ out[(b.0-bounds.0.0) as usize] = 'B'}
    }

    for s in sensors.iter(){
        if s.1 == row{ out[(s.0-bounds.0.0) as usize] = 'S'}
        if (row < s.1 - s.2) || (row > s.1 + s.2){continue}
        let dist = s.2 - (s.1 - row).abs();
        for idx in (s.0 - dist)..(1 + s.0 + dist) {
            if out[(idx-bounds.0.0) as usize] == '.'{
                out[(idx-bounds.0.0) as usize] = '#';
        }}
    }
    out
}


pub(crate) fn part_1() {
    let (sensors, beacons) = read_input();
    let bounds = calc_bounds(&sensors);
    let mut visible_count = 0;
    use std::time::Instant;
    let _now = Instant::now();
    // let r = string_row(&sensors, &beacons, 10,&bounds);
    // for i in 0..21 {
    //     print!("{i: >4}: ");
    //     let r = string_row(&sensors, &beacons, i, &bounds);
    //     println!("{}", r.clone().into_iter().collect::<String>());
    // }

    let r = string_row(&sensors, &beacons, 2000000,&bounds);
    for c in r.into_iter(){
        if (c!='.') & (c!='B'){visible_count += 1}
    }
    println!("Day {DAY} Part 1: {visible_count}");
}

fn overlap_all_start_stop(in_v: &[(i64, i64)], bounds: &((i64, i64), (i64, i64)))-> Vec<(i64, i64)>{
    let mut new_v = vec![];
    for t in in_v.iter(){
        if (t.1 < bounds.0.0) || (t.0 > bounds.0.1){
            continue
        } else { new_v.push(*t)}
    }
    new_v.sort_by(|a,b| a.0.cmp(&b.0));
    //print!("{new_v:?} -> ");
    let mut out_v = vec![new_v[0]];
    let mut idx = 0;
    for (_, t) in new_v.into_iter().enumerate().skip(1){
        if t.0 <= out_v[idx].1 + 1 { out_v[idx].1 = max(t.1, out_v[idx].1)}
        else{ out_v.push(t); idx +=1}
    }
    //print!("{out_v:?}   ");
    out_v
}

fn find_possible_positions(sensors: &[(i64, i64, i64)], row: i64, bounds: &((i64, i64), (i64, i64))) -> Vec<(i64, i64)>{
    if (row < bounds.1.0) || (row > bounds.1.1){panic!("Row {row} is outside of bounds {bounds:?}")}
    let mut excluded :Vec<(i64, i64)> = vec![];

    for s in sensors.iter(){
        if (row < s.1 - s.2) || (row > s.1 + s.2){continue}
        let dist = s.2 - (s.1 - row).abs();
        excluded.push((s.0 - dist, s.0 + dist));
    }
    let condensed = overlap_all_start_stop(&excluded, bounds);
    let mut out = vec![];
    match condensed.len(){
        0 => println!("There is no way you have NO coverage."),
        1 => if (condensed[0].0 <= bounds.0.0) & (condensed[0].1 >= bounds.0.1) {return out},
        2 => {for i in (condensed[0].1 + 1)..condensed[1].0{
            out.push((i, row));
        }; return out
        },
        _ => panic!{"How do you have more than 2 non-overlapping regions?!\n{:?}", condensed}
    }

    out
}

pub(crate) fn part_2() {
    let (sensors, _beacons) = read_input();
    let max_val = 4000000i64;
    // let max_val = 20i64;
    let bounds:((i64, i64), (i64, i64)) = ((0,max_val),(0,max_val));

    use std::time::Instant;
    let now = Instant::now();
    // Rather that building a big string, we can get all the ranges and
    // see if there are any missing numbers in the range.
    for i in bounds.1.0..(bounds.1.1+1){
        let o = find_possible_positions(&sensors, i, &bounds);
        if o.len() == 1 {
            println!("Day {DAY} Part 2: {}", o[0].0 * 4000000 + o[0].1);
            break;
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed); // 6.71 seconds
}
