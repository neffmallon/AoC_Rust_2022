use std::collections::HashMap;
use regex::Regex;
use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 21;
// problem at https://adventofcode.com/2022/day/21

enum MathOp{Add, Sub, Mult, Div}

struct MathMonkey{
    name: String,
    a: String,
    b: String,
    op: MathOp,
}

fn str_to_op(s: &str) -> MathOp{
    match s{
        "+" => MathOp::Add,
        "-" => MathOp::Sub,
        "*" => MathOp::Mult,
        "/" => MathOp::Div,
        _ => panic!("That's no a math operation!"),
    }
}

fn read_and_sort_input() -> (HashMap<String, i64>, HashMap<String, MathMonkey>) {
    let mut num_monkeys: HashMap<String, i64> = HashMap::with_capacity(2520);
    let mut math_monkeys: HashMap<String, MathMonkey> = HashMap::with_capacity(2519-1260);
    if let Ok(lines) = read_day_input_lines(DAY) {
        let num_re = Regex::new(r"(\w+): (\d+)").unwrap();
        let math_re = Regex::new(r"(\w+): (\w+) (\S) (\w+)").unwrap();

        'lines: for line in lines.flatten() {
            for (_, [name, num]) in num_re.captures_iter(&line).map(|c| c.extract()) {
                // if there is a match, add it to the num_monkeys HashMap
                if name == "humn"{
                    math_monkeys.insert(
                    name.to_string(),
                    MathMonkey{name:name.to_string(), a: "a".to_string(), b:"b".to_string(), op: MathOp::Add}
                );
                }
                num_monkeys.insert(name.to_string(), num.parse::<i64>().unwrap());
                continue 'lines
            }
            // if there wasn't a match, then we have to parse and add the MathMonkey
            for (_, [name, a, op, b]) in math_re.captures_iter(&line).map(|c| c.extract()) {
                // if there is a match, add it to the num_monkeys HashMap
                math_monkeys.insert(
                    name.to_string(),
                    MathMonkey{name:name.to_string(), a: a.to_string(), b:b.to_string(), op: str_to_op(op)}
                );
                continue 'lines
            }
        }
    }
    (num_monkeys, math_monkeys)
}

fn resolve_math_monkey(
    monkey: &MathMonkey,
    num_monkeys: &mut HashMap<String, i64>,
    math_monkeys: &HashMap<String, MathMonkey>
)-> i64{
    let a: i64;
    let b: i64;
    // get a
    if num_monkeys.contains_key(&monkey.a){
        a = *num_monkeys.get(&monkey.a).unwrap()
    } else {
        a = resolve_math_monkey(
            &math_monkeys.get(&monkey.a).unwrap(), num_monkeys, math_monkeys
        )
    }
    // get b
    if num_monkeys.contains_key(&monkey.b){
        b = *num_monkeys.get(&monkey.b).unwrap()
    } else {
        b = resolve_math_monkey(
            &math_monkeys.get(&monkey.b).unwrap(), num_monkeys, math_monkeys
        )
    }
    let num = match monkey.op{
        MathOp::Add  => a + b,
        MathOp::Sub  => a - b,
        MathOp::Mult => a * b,
        MathOp::Div  => a / b,
    };
    num_monkeys.insert(monkey.name.clone(), num);
    return num
}

pub(crate) fn part_1() {
    let (mut num_monkeys, mut math_monkeys) = read_and_sort_input();
    let ans = resolve_math_monkey(math_monkeys.get("root").unwrap(), &mut num_monkeys, &math_monkeys);
    println!("Day {DAY} Part 1: {}", ans);
}

fn find_humn_path(
    monkey: &MathMonkey,
    math_monkeys: &HashMap<String, MathMonkey>,
    current_vec: &mut Vec<String>,
    target: &mut String
){
    if monkey.name == "humn"{return}
    if math_monkeys.contains_key(&monkey.a){
        find_humn_path(
            math_monkeys.get(&monkey.a).unwrap(), math_monkeys, current_vec, target
        )
    }
    if math_monkeys.contains_key(&monkey.b){
        find_humn_path(
            math_monkeys.get(&monkey.b).unwrap(), math_monkeys, current_vec, target
        )
    }
    let mut change_target = false;
    if &monkey.a == target{
        current_vec.push(monkey.a.to_string());
        change_target = true
    } else if &monkey.b == target{
        current_vec.push(monkey.b.to_string());
        change_target = true
    }
    if change_target{*target = monkey.name.clone()}
}

fn solve_for_x(
    value: i64,
    monkey: &MathMonkey,
    num_monkeys: &mut HashMap<String, i64>,
    math_monkeys: &HashMap<String, MathMonkey>,
    target: &str,
) -> i64 {
    let other;
    let other_first: bool;
    // resolve the number of "other"
    if monkey.b == target{
        other = if num_monkeys.contains_key(&monkey.a){
            *num_monkeys.get(&monkey.a).unwrap()
        } else {
            resolve_math_monkey(
                math_monkeys.get(&monkey.a).unwrap(), num_monkeys, math_monkeys
            )
        };
        other_first=true;
    } else if monkey.a == target{
        other = if num_monkeys.contains_key(&monkey.b){
            *num_monkeys.get(&monkey.b).unwrap()
        } else {
            resolve_math_monkey(
                math_monkeys.get(&monkey.b).unwrap(), num_monkeys, math_monkeys
            )
        };
        other_first=false;
    } else {panic!("For Monkey {}, neither a {} nor b {} is target {}", monkey.name, monkey.a, monkey.b, target)}

    //println!("value: {value}, other: {other}, other_first: {other_first}");
    return if other_first{
        match monkey.op{
            MathOp::Add  => value - other,
            MathOp::Sub  => other - value,
            MathOp::Mult => value / other,
            MathOp::Div  => other / value,
        }
    } else {
        match monkey.op{
            MathOp::Add  => value - other,
            MathOp::Sub  => value + other,
            MathOp::Mult => value / other,
            MathOp::Div  => value * other,
        }
    }
}

pub(crate) fn part_2() {
    let (mut num_monkeys, mut math_monkeys) = read_and_sort_input();
    num_monkeys.remove("humn");
    let mut humn = "humn".to_string();
    let mut path_to_humn = Vec::new();
    find_humn_path(math_monkeys.get("root").unwrap(), &math_monkeys, &mut path_to_humn, &mut humn);
    //println!("{:?}", path_to_humn);
    let mut monkey = math_monkeys.get("root").unwrap();
    let mut answer;
    if monkey.a == path_to_humn[path_to_humn.len()-1]{
        answer = resolve_math_monkey(math_monkeys.get(&monkey.b).unwrap(), &mut num_monkeys, &math_monkeys);
        monkey = math_monkeys.get(&monkey.a).unwrap()
    } else{
        answer = resolve_math_monkey(math_monkeys.get(&monkey.a).unwrap(), &mut num_monkeys, &math_monkeys);
        monkey = math_monkeys.get(&monkey.b).unwrap();
    };
    //println!("vglr = {}", answer);
    for target in path_to_humn.into_iter().rev().skip(1){
        answer = solve_for_x(
            answer, monkey, &mut num_monkeys, &math_monkeys, &target
        );
        //println!("target {target} = {answer}");
        if target == "humn"{break}
        match math_monkeys.get(&target){
            Some(m) => monkey = m,
            None => panic!("Monkey {} was not found in math_monkeys (len: {})!", target, math_monkeys.len()),
        };
    }
    (num_monkeys, math_monkeys) = read_and_sort_input();
    num_monkeys.insert("humn".to_string(), answer);
    let qhbp = resolve_math_monkey(math_monkeys.get("qhbp").unwrap(), &mut num_monkeys, &math_monkeys);
    let vglr = resolve_math_monkey(math_monkeys.get("vglr").unwrap(), &mut num_monkeys, &math_monkeys);
    if qhbp == vglr{ println!("We did it! Day {DAY} Part 1: {answer}")}
    else{ println!("Not quite! {} =/= {}",qhbp , vglr )}
}

#[cfg(test)]
mod tests {
    use crate::day_21::{read_and_sort_input, resolve_math_monkey};
    #[test]
    fn part_1() {
        let (mut num_monkeys, mut math_monkeys) = read_and_sort_input();
        let ans = resolve_math_monkey(math_monkeys.get("root").unwrap(), &mut num_monkeys, &math_monkeys);
        assert_eq!(ans, 155708040358220);
    }
}
