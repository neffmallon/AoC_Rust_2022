use std::collections::HashMap;

use crate::general_helpers::read_day_input_lines;



const DAY: u8 = 7;
// problem at https://adventofcode.com/2022/day/7
// For reading inputs, I need to implement "ls", "cd ..",
// "cd <dir_name>", "dir <dir_name>", <size> <filename>
// I have given up for now on constructing my own DAG

fn full_dir_name(v: &[String]) -> String{
    v.join("-").clone()
}

pub(crate) fn part_1() {
    let mut all_dirs_sizes: HashMap<String, u64> = HashMap::new();
    let mut current_dirs = vec![];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            //println!("{}",line);
            if line.contains("$ ls"){
                // We just pass over the ls command
                continue
            } else if line.contains("dir ") {
                // let binding = line.clone();
                // let (_, name) = binding.trim().split_at(line.find(" ").unwrap());
                // all_dirs_sizes.insert(name.trim().to_string(), 0);
                //println!("{}",line);
            } else if line[0..4].trim() == "$ cd" {
                // modify the current_dir_idx to the target directory
                let target_dir_name = line.trim().split_at(4).1.trim();
                // println!("Target name: '{}'",target_dir_name);
                if target_dir_name == ".."{
                    current_dirs.truncate(current_dirs.len() - 1)
                } else{
                    current_dirs.push(target_dir_name.trim().to_string())
                }
            } else {
                let (file_size_str, _) = line.trim().split_at(line.find(' ').unwrap());
                let file_size: u64 = match file_size_str.trim().parse::<u64>() {
                            Ok(num)=>num,
                            Err(_) => {
                                println!("error at '{}'", line);
                                0
                            },
                        };
                for end_idx in 0..current_dirs.len(){
                    // println!("Current Directories! '{}'", dir_name);
                    all_dirs_sizes.entry(full_dir_name(&current_dirs[0..(end_idx+1)]))
                        .and_modify(|s| *s += file_size)
                        .or_insert(file_size);
                }
            }
        }
    }
    let mut sum_small_directories: u64 = 0;
    for value in all_dirs_sizes.values(){
        if *value <= 100000{
            sum_small_directories += value
        }
        // println!("Day {DAY} dir '{}': {}",key, value)
    }
    //println!("Day {DAY} dir {} size: {}","/", all_dirs_sizes.get("/").unwrap());
    println!("Day {DAY} part 1: total sum of small directories: {}",sum_small_directories);

    let needed_size = 30000000 - (70000000 - all_dirs_sizes.get("/").unwrap()) ;
    let mut smallest_dir_size = all_dirs_sizes.get("/").unwrap();

    for value in all_dirs_sizes.values(){
        if (value < smallest_dir_size) & (value >= &needed_size) {
            smallest_dir_size = value
        }
    }

    println!("Day {DAY} part 2: smallest directory with needed space: {}",smallest_dir_size);
}

// task completed in part 1
pub(crate) fn part_2() {}