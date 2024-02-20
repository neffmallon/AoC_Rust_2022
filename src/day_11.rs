use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 11;
// problem at https://adventofcode.com/2022/day/11
#[derive(Clone)]
enum OpType{
    Add,
    Mult,
    Square,
}
#[derive(Clone)]
struct Monkey{
    items: Vec<usize>,
    op_type: OpType,
    op_num: usize,
    test_num: usize,
    true_idx: usize,
    false_idx: usize,
    inspect_count: usize,
}

fn read_monkey(monkey_lines: &[String])-> Monkey{

    let item_string= monkey_lines[1].split(':').collect::<Vec<&str>>()[1];
    let items = if item_string.contains(','){
        item_string.trim().split(", ").collect::<Vec<&str>>().iter()
        .map(|s| s.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>()
    } else {
        vec![item_string.trim().parse::<usize>().unwrap()]
    };
    // println!("{:?}",items);
    let mut op_type = OpType::Add;
    let mut op_num = 1usize;

    // get the operation type and number
    if monkey_lines[2].contains("old * old"){
        op_type = OpType::Square;
        op_num = 1usize;
    }
    else if monkey_lines[2].contains('*'){
        op_type = OpType::Mult;
        op_num = monkey_lines[2].split('*').collect::<Vec<&str>>()[1].trim().parse::<usize>().unwrap();
    }
    else if monkey_lines[2].contains('+') {
        op_type = OpType::Add;
        op_num = monkey_lines[2].split('+').collect::<Vec<&str>>()[1].trim().parse::<usize>().unwrap();
    }
    else {panic!("Unknown Operation:'{}'", monkey_lines[2])};
    // println!("op num: {}", op_num);

    let test_num = monkey_lines[3].split("by ").collect::<Vec<&str>>()[1].trim().parse::<usize>().unwrap();
    // println!("test_num: {}", test_num);
    let true_idx = monkey_lines[4].split("monkey ").collect::<Vec<&str>>()[1].trim().parse::<usize>().unwrap();
    // println!("true_idx: {}", true_idx);
    let false_idx = monkey_lines[5].split("monkey ").collect::<Vec<&str>>()[1].trim().parse::<usize>().unwrap();
    // println!("false_idx: {}", false_idx);
    Monkey{
        items,
        op_type,
        op_num,
        test_num,
        true_idx,
        false_idx,
        inspect_count:0,
    }
}

fn process_monkeys(monkey_vector: &[Monkey], target_idx: usize) -> Vec<Monkey>{
    let true_idx = monkey_vector[target_idx].true_idx;
    let false_idx = monkey_vector[target_idx].false_idx;
    let mut process_monkey = monkey_vector[target_idx].clone();
    let mut true_monkey = monkey_vector[true_idx].clone();
    let mut false_monkey = monkey_vector[false_idx].clone();

    for item in process_monkey.items.iter(){
        process_monkey.inspect_count += 1;
        let test_item = match process_monkey.op_type{
            OpType::Square => (item * item)/3,
            OpType::Add => (item + process_monkey.op_num)/3,
            OpType::Mult => (item * process_monkey.op_num)/3,
        };
        if test_item%process_monkey.test_num == 0{
            true_monkey.items.push(test_item)
        } else{
            false_monkey.items.push(test_item)
        }
    }
    process_monkey.items = vec![];

    let mut out_vec = vec![];
    for (i, monkey) in monkey_vector.iter().enumerate(){
        if i == target_idx {out_vec.push(process_monkey.clone())}
        else if i == true_idx { out_vec.push(true_monkey.clone())}
        else if i == false_idx { out_vec.push(false_monkey.clone())}
        else {out_vec.push(monkey.clone())}
    }
    out_vec

}
pub(crate) fn part_1() {
    let mut monkey_lines: Vec<String> = vec![];
    let mut monkeys: Vec<Monkey> = vec![];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.trim() == "" {
                monkeys.push(read_monkey(&monkey_lines));
                monkey_lines = vec![];
            } else{
                monkey_lines.push(line.to_string())
            }
        }
    }
    println!("All monkeys loaded!");
    // process the monkeys!
    for _round in 0..20{
        for idx in 0..monkeys.len(){
        monkeys = process_monkeys(&monkeys, idx)
        }
    }


    // let mut soreted_monkeys = monkeys.clone();
    // soreted_monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    //
    // println!{"Part 2: Total Monkey Business = {}",
    //     soreted_monkeys[monkeys.len()-1].inspect_count * soreted_monkeys[monkeys.len()-2].inspect_count
    // }
    for monkey in monkeys.iter(){
        println!{"Part 1: Monkey inspeted items {} times", monkey.inspect_count}
    }
}

fn process_mod_monkeys(monkey_vector: &[Monkey], target_idx: usize) -> Vec<Monkey>{
    let modulus = monkey_vector.iter()
        .map(|m| m.test_num).product::<usize>();
    let true_idx = monkey_vector[target_idx].true_idx;
    let false_idx = monkey_vector[target_idx].false_idx;
    let mut process_monkey = monkey_vector[target_idx].clone();
    let mut true_monkey = monkey_vector[true_idx].clone();
    let mut false_monkey = monkey_vector[false_idx].clone();

    for item in process_monkey.items.iter(){
        process_monkey.inspect_count += 1;
        let test_item = match process_monkey.op_type{
            OpType::Square => (item * item)% modulus,
            OpType::Add => (item + process_monkey.op_num)% modulus,
            OpType::Mult => (item * process_monkey.op_num)% modulus,
        };
        if test_item%process_monkey.test_num == 0{
            true_monkey.items.push(test_item)
        } else{
            false_monkey.items.push(test_item)
        }
    }
    process_monkey.items = vec![];

    let mut out_vec = vec![];
    for (i, monkey) in monkey_vector.iter().enumerate(){
        if i == target_idx {out_vec.push(process_monkey.clone())}
        else if i == true_idx { out_vec.push(true_monkey.clone())}
        else if i == false_idx { out_vec.push(false_monkey.clone())}
        else {out_vec.push(monkey.clone())}
    }
    out_vec

}

pub(crate) fn part_2() {
    let mut monkey_lines: Vec<String> = vec![];
    let mut monkeys: Vec<Monkey> = vec![];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.trim() == "" {
                monkeys.push(read_monkey(&monkey_lines));
                monkey_lines = vec![];
            } else{
                monkey_lines.push(line.to_string())
            }
        }
    }
    println!("All monkeys loaded!");
    // process the monkeys!
    for _round in 0..10000{
        for idx in 0..monkeys.len(){
        monkeys = process_mod_monkeys(&monkeys, idx)
        }
    }
//     let mut soreted_monkeys = monkeys.clone();
//     soreted_monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
//
//     println!{"Part 2: Total Monkey Business = {}",
//         soreted_monkeys[monkeys.len()-1].inspect_count * soreted_monkeys[monkeys.len()-2].inspect_count
//     }

    for monkey in monkeys.iter(){
        println!{"Part 2: Monkey inspeted items {} times", monkey.inspect_count}
    }

}
