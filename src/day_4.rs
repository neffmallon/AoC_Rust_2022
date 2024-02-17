use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 4;

struct ElfPairs {
    elf_1_start: u32,
    elf_1_end: u32,
    elf_2_start: u32,
    elf_2_end: u32,
}

fn has_complete_overlap(elfs: ElfPairs) -> bool {
    let one_in_two: bool = {
        elfs.elf_2_start <= elfs.elf_1_start } &
        { elfs.elf_2_end >= elfs.elf_1_end };
    let two_in_one: bool = {
        elfs.elf_1_start <= elfs.elf_2_start } &
        { elfs.elf_1_end >= elfs.elf_2_end };
    one_in_two | two_in_one
}

fn has_any_overlap(elfs: ElfPairs) -> bool {

    let start_in_two: bool = {
        elfs.elf_1_start >= elfs.elf_2_start } &
        { elfs.elf_1_start <= elfs.elf_2_end };
    let end_in_two: bool = {
        elfs.elf_1_end >= elfs.elf_2_start } &
        { elfs.elf_1_end <= elfs.elf_2_end };
    return has_complete_overlap(elfs) | start_in_two | end_in_two;
}

fn parse_elfs(l: &String) -> ElfPairs{
    // string should be in the form 1-93,2-11
    let elfs = l.split(",").collect::<Vec<&str>>();
    let elf_1 = elfs[0].split("-").collect::<Vec<&str>>();
    let elf_2 = elfs[1].split("-").collect::<Vec<&str>>();
    ElfPairs {
    elf_1_start: elf_1[0].parse().expect(&*format!("failed to convert {}", elf_1[0])),
    elf_1_end: elf_1[1].parse().expect(&*format!("failed to convert {}", elf_1[1])),
    elf_2_start: elf_2[0].parse().expect(&*format!("failed to convert {}", elf_2[0])),
    elf_2_end: elf_2[1].parse().expect(&*format!("failed to convert {}", elf_2[1])),
}
}

pub(crate) fn part_1() {
    let mut contained_count:u32 =0;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten(){
            let elf_pair = parse_elfs(&line.trim().to_string());
            if has_complete_overlap(elf_pair){
                contained_count +=1
            }
        }
    }
    println!("Day {DAY} Part 1: {contained_count}")
}

pub(crate) fn part_2() {
        let mut contained_count:u32 =0;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten(){
            let elf_pair = parse_elfs(&line.trim().to_string());
            if has_any_overlap(elf_pair){
                contained_count +=1
            }
        }
    }
    println!("Day {DAY} Part 2: {contained_count}")
}