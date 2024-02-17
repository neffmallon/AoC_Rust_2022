use crate::general_helpers::read_day_input_lines;


const DAY: u8 = 10;
// problem at https://adventofcode.com/2022/day/10

fn check_cycle(x: &i32, cycle_count: &i32, next_cycle_check: &i32, final_sum: &i32)-> (i32, i32){
    if cycle_count == next_cycle_check{
        println!("{} added to total for a new sum of {}", x * cycle_count, final_sum + x * cycle_count);
        return (next_cycle_check + 40, final_sum + x * cycle_count)
    }
    return (next_cycle_check.clone(), final_sum.clone())
}

pub(crate) fn part_1() {
    let mut x = 1;
    let mut cycle_count = 1;
    let mut next_cycle_check = 20;
    let mut final_sum = 0;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.trim() == "noop"{
                // cycle start
                (next_cycle_check, final_sum) = check_cycle(&x, &cycle_count, &next_cycle_check, &final_sum);
                cycle_count += 1; // cycle end
            } else if &line.trim()[0..4] =="addx"{
                // cycle start
                (next_cycle_check, final_sum) = check_cycle(&x, &cycle_count, &next_cycle_check, &final_sum);
                cycle_count += 1;// cycle end
                // cycle start
                (next_cycle_check, final_sum) = check_cycle(&x, &cycle_count, &next_cycle_check, &final_sum);
                cycle_count += 1; // cycle end
                let n =  match line.trim()
                    .split_at(line.find(" ").unwrap()).1.trim()
                    .parse::<i32>()
                { Ok(num) => num,
                Err(_) => {
                    println!("{}", line);
                    panic!()
                },};
                x += n;
            }
        if next_cycle_check > 220{break}
        }
    }
    println!("Day {DAY} part 1: {}",final_sum);
}

fn draw_pix(cycle: &i32, x: &i32)-> bool{
    if cycle > &240 {return false}
    return ((cycle - 1)%40 - x).abs() <= 1
}

pub(crate) fn part_2() {
    let mut x = 1;
    let mut cycle_count = 1;
    let mut display = vec![vec![' ';40];6];

    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.trim() == "noop"{
                // cycle start
                if draw_pix(&cycle_count, &x){
                    display[((cycle_count-1)/40i32) as usize][((cycle_count - 1)%40) as usize] = '#';
                }
                cycle_count += 1; // cycle end
            } else if &line.trim()[0..4] =="addx"{
                // cycle start
                if draw_pix(&cycle_count, &x){
                    display[((cycle_count-1)/40i32) as usize][((cycle_count - 1)%40) as usize] = '#';
                }
                cycle_count += 1;// cycle end
                // cycle start
                if draw_pix(&cycle_count, &x){
                    display[((cycle_count-1)/40i32) as usize][((cycle_count - 1)%40) as usize] = '#';
                }
                cycle_count += 1; // cycle end
                let n =  match line.trim()
                    .split_at(line.find(" ").unwrap()).1.trim()
                    .parse::<i32>()
                { Ok(num) => num,
                Err(_) => {
                    println!("{}", line);
                    panic!()
                },};
                x += n;
            }
        if cycle_count > 240{break}
        }
    }
    for d in display.iter(){
        println!("{}",d
            .iter()
            .collect::<String>()
        )
    }
}