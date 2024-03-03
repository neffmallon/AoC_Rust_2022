use std::cmp::{max, min};
use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 20;
// problem at https://adventofcode.com/2022/day/20
// Mixing the signals
// the real challenge is going to be doing this efficiently.
// We can either access quickly over current index or original index.
// Vecs store all their items sequentially in memory, so removing an item from
// the middle and putting it elsewhere seems like it will be slower than just conditionally
// updating the "current_index" again.

#[derive(Clone, Debug)]
struct Part{
    instruction: i64,
    current_idx: i64,
}

fn read_input() -> Vec<Part>{
    let mut out: Vec<Part> = Vec::with_capacity(5010);
    if let Ok(lines) = read_day_input_lines(DAY) {
        for (idx, line) in lines.flatten().enumerate() {
            if line.trim() == ""{break}
            out.push(Part{
                instruction: line.trim().parse::<i64>().unwrap(),
                current_idx: idx as i64,
            })
        }
    }
    out
}

fn coerce_destination(current_idx: i64, instruction: i64, length: i64)->i64{

    let mut destination_idx = current_idx + instruction;
    if instruction == 8{
        println!("inst: {instruction}, destination: {destination_idx}")
    }
    destination_idx =  if destination_idx < 0 {
        (destination_idx.abs()/(length - 1))*(length - 1) + destination_idx
    } else { destination_idx + (1 - length) * (destination_idx/(length - 1))};

    while destination_idx < 0 || destination_idx >= length{
      destination_idx =  if destination_idx < 0 {destination_idx + length - 1}
      else { destination_idx + 1 - length};
    }
    destination_idx
}

fn do_step(v: &mut [Part], tartet_original_idx: usize){
    let part = v[tartet_original_idx].clone();
    let current_idx = part.current_idx;
    let l = v.len() as i64;
    let destination_idx = coerce_destination(current_idx, part.instruction, l);
    let big = max(destination_idx, current_idx);
    let sml = min(destination_idx, current_idx);
    let increment = if current_idx < destination_idx{-1}else{1};
    for idx in 0..v.len(){
        if v[idx].current_idx < sml{continue}
        if v[idx].current_idx > big{continue}
        if v[idx].current_idx == current_idx{v[idx].current_idx = destination_idx}
        else{v[idx].current_idx += increment}
        if v[idx].current_idx >= l{panic!("Something has gone terribly wrong!")}
        if v[idx].current_idx < 0{
            panic!("Oh no {:?}, you are too smol!", v[idx])
        }
    }
}

fn re_create_vector(v: &[Part])->Vec<i64>{
    let mut out = vec![0;v.len()];
    for p in v.iter(){
        out[p.current_idx as usize] = p.instruction
    }
    out
}

fn find_coords(v: &[Part])->i64{
    let mut s = 0;
    let l = v.len() as i64;
    let mut start_idx = 0;
    for p in v.iter(){
        if p.instruction == 0{
            start_idx = p.current_idx;
            //println!("Current index of 0 is {}", p.current_idx);
            break
        }
    }
    for p in v.iter() {
        if p.current_idx == (start_idx+1000) % l {
            println!("idx 1000 is {}", p.instruction);
            s += p.instruction
        } else if p.current_idx == (start_idx+2000) % l {
            println!("idx 2000 is {}", p.instruction);
            s += p.instruction
        } else if p.current_idx == (start_idx+3000) % l {
            println!("idx 3000 is {}", p.instruction);
            s += p.instruction
        }
    }
    s
}

fn do_full_mix(v: &mut [Part]){
    for idx in 0..v.len(){
        do_step(v, idx)
    }
}

pub(crate) fn part_1() {
    let mut order_vec = read_input();
    do_full_mix(&mut order_vec);
    //println!("{:?}", re_create_vector(&order_vec));
    println!("Day {DAY} Part 1: {:?}", find_coords(&order_vec)); // 16533 is correct
}

pub(crate) fn part_2() {
    let mut order_vec = read_input();
    let key = 811589153;
    for idx in 0..order_vec.len(){
        order_vec[idx].instruction *= key
    }
    for _ in 0..10{
        do_full_mix(&mut order_vec)
    }
    println!("Day {DAY} Part 2: {:?}", find_coords(&order_vec));
}

#[cfg(test)]
mod tests {
    use crate::day_20::{do_full_mix, do_step, Part, re_create_vector};

    fn reindex_to_zero(v: &[i64])-> Vec<i64>{
        let zero_idx = v.iter().position(|&n| n == 0).unwrap();
        let mut out = Vec::with_capacity(v.len());
        out.extend(&v[zero_idx..]);
        out.extend(&v[..zero_idx]);
        out
    }

    #[test]
    fn simple_sort() {
        let input: Vec<i64> = vec![1, 2, -3, 3, -2, 0, 4];
        let mut v = vec![];
        for (idx, inst) in input.iter().enumerate(){
            v.push(Part{
                instruction: *inst,
                current_idx: idx as i64,
            })
        }
        do_full_mix(&mut v);
        assert_eq!(reindex_to_zero(&re_create_vector(&v)), reindex_to_zero(&[1, 2, -3, 4, 0, 3, -2]));
    }

    #[test]
    fn part_2_sort() {
        let input: Vec<i64> = vec![1, 2, -3, 3, -2, 0, 4];
        let key = 811589153;
        let mut v = vec![];
        for (idx, inst) in input.iter().enumerate(){
            v.push(Part{
                instruction: *inst * key,
                current_idx: idx as i64,
            })
        }
        for _ in 0..10 {
            do_full_mix(&mut v);
        }
        assert_eq!(reindex_to_zero(&re_create_vector(&v)), reindex_to_zero(&[0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153]));
    }

    #[test]
    fn ones_sort() {
        let input: Vec<i64> = vec![0, -1, -1, 1];
        let mut v = vec![];
        for (idx, inst) in input.iter().enumerate(){
            v.push(Part{
                instruction: *inst,
                current_idx: idx as i64,
            })
        }
        do_full_mix(&mut v);
        assert_eq!(reindex_to_zero(&re_create_vector(&v)), reindex_to_zero(&[-1, 1, -1, 0]));
    }

    #[test]
    fn bigger_sort() {
        let input: Vec<i64> = vec![1, 2, -3, 3, -2, 0, 8];
        let mut v = vec![];
        for (idx, inst) in input.iter().enumerate(){
            v.push(Part{
                instruction: *inst,
                current_idx: idx as i64,
            })
        }
        do_full_mix(&mut v);
        assert_eq!(reindex_to_zero(&re_create_vector(&v)), reindex_to_zero(&[1, 8, 2, -3, 0, 3, -2]));
    }

    #[test]
    fn test_reindex_to_zero() {
        let input: Vec<i64> = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(reindex_to_zero(&input), vec![0, 4, 1, 2, -3, 3, -2]);
    }

    #[test]
    fn test_recreate_vector() {
        let input: Vec<i64> = vec![1, 2, -3, 3, -2, 0, 4];
        let mut v = vec![];
        for (idx, inst) in input.iter().enumerate(){
            v.push(Part{
                instruction: *inst,
                current_idx: idx as i64,
            })
        }
        assert_eq!(re_create_vector(&v), input);
    }
}
