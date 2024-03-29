use std::cmp::{max, Ordering};
use regex::Regex;

use std::collections::{HashMap, HashSet};
use rand::{Rng, thread_rng};
use rand::distributions::WeightedIndex;
use rand::distributions::Distribution;
use rand::prelude::SliceRandom;

use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 19;
// problem at https://adventofcode.com/2022/day/19

////////////////////////////////// Making Strategies ///////////////////////////////////

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Organism(Vec<u8>);

const GENOME_SIZE: usize = 20;

fn repair_genes(genes: &mut Vec<u8>) {
    if !genes.contains(&3u8){
        for _ in 0..5{
            genes.insert(10,3)
        }
    }
    let first_three = genes.iter().position(|n| *n == 3).unwrap();
    if !genes.contains(&2u8) {
        genes.insert(first_three, 2);
    } else {
        let first_two = genes.iter().position(|n| *n == 2).unwrap();
        if first_two > first_three {
            genes.insert(first_three, 2u8);
        }
    }
    let first_two = genes.iter().position(|n| *n == 2).unwrap();
    if !genes.contains(&1u8) {
        genes.insert(first_two, 2);
    } else {
        let first_one = genes.iter().position(|n| *n == 1).unwrap();
        if first_one > first_two {
            genes.insert(first_two, 1u8);
        }
    }
    genes.truncate(GENOME_SIZE);
}

fn generate_organism()-> Organism{
    let mut out_vec = vec![];
    for _ in 0..3{
        out_vec.push(rand::thread_rng().gen_range(0..=1) as u8)
    }
    for _ in 0..5{
        out_vec.push(rand::thread_rng().gen_range(1..=2) as u8)
    }
    for _ in 0..5{
        out_vec.push(rand::thread_rng().gen_range(2..=3) as u8)
    }
    repair_genes(&mut out_vec);
    Organism(out_vec)
}

fn mate_pair(a: &Organism, b: &Organism) -> Vec<Organism>{
    let mut out = Vec::new();
    for idx in 1..GENOME_SIZE-1{
        let mut o = vec![];
        o.extend(a.0.iter().take(idx));
        o.extend(b.0.iter().skip(idx));
        repair_genes(&mut o);
        out.push(Organism(o));

        let mut o = vec![];
        o.extend(b.0.iter().take(idx));
        o.extend(a.0.iter().skip(idx));
        repair_genes(&mut o);
        out.push(Organism(o));

    }
    out
}

/// Generates mutated versions of a bunch of child organisms
fn mutate_babies(babies: &[Organism]) -> Vec<Organism>{
    let mut out = vec![];
    for baby in babies.iter(){
        let mut m = baby.0.clone();
        let add = rand::thread_rng().gen_range(0..=1);
        let idx = rand::thread_rng().gen_range(0..m.len());
        if add == 0{
            if m[idx] == 0{
                m[idx] +=1
            } else{m[idx] -= 1}
        } else {
            if m[idx] == 3{
                m[idx] -=1
            } else{m[idx] += 1}
        }
        repair_genes(&mut m);
        out.push(Organism(m))
    }
    out
}



///////////////////////////////////// Computing Fitness ///////////////////////////////////
#[derive(Copy, Clone, Debug)]
struct Blueprint{
    idx: u8,
    ore_cost: u8, // What an ore robot costs in ore
    cly_cost: u8, // what a clay robot costs in ore
    obs_ore_cost: u8, // what an obsidian robot costs in ore
    obs_cly_cost: u8, // what an obsidian robot costs in clay
    geo_ore_cost: u8, // what a geode robot costs in ore
    geo_obs_cost: u8, // what a geode robot costs in obsidian
}
#[derive(PartialEq)]
enum Robots{
    Ore,
    Clay,
    Obsidian,
    Geo,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct State{
    turn: u8,
    resources: Resources,
    bots: Resources,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Resources {
    ore: u8,
    cly: u8,
    obs: u8,
    geo: u8,
}

const START: State = State{
    turn: 0,
    resources: Resources{ore:0,cly:0,obs:0,geo:0},
    bots: Resources{ore:1,cly:0,obs:0,geo:0},
};

fn read_blueprint(s: &str)-> Blueprint{
    let re = Regex::new(r"\d+").unwrap();
    let mut caps: Vec<u8> = vec![];
    for (num, []) in re.captures_iter(s).map(|c| c.extract()) {
        caps.push(num.parse::<u8>().unwrap());
    }
    Blueprint{
        idx: caps[0],
        ore_cost: caps[1],
        cly_cost: caps[2],
        obs_ore_cost: caps[3],
        obs_cly_cost: caps[4],
        geo_ore_cost: caps[5],
        geo_obs_cost: caps[6],
    }
}

fn ints_to_bots(v: &[u8])-> Vec<Robots>{
    let mut out = vec![];
    for i in v.into_iter(){
        out.push(
            match i {
                0 => Robots::Ore,
                1 => Robots::Clay,
                2 => Robots::Obsidian,
                3 => Robots::Geo,
                _ => panic!("That's not a valid robot number")
            }
        )
    }
    out
}

fn spend_resources(state: &mut State, blueprint: &Blueprint, bot: u8){
        match bot{
        0 => state.resources.ore -= blueprint.ore_cost,
        1 => state.resources.ore -= blueprint.cly_cost,
        2 => {state.resources.ore -= blueprint.obs_ore_cost; state.resources.cly -= blueprint.obs_cly_cost},
        3 => {state.resources.ore -= blueprint.geo_ore_cost; state.resources.obs -= blueprint.geo_obs_cost},
        _ => panic!("Not a bot!")
    }
}

fn collect_resources(state: &mut State){
    state.resources.ore += state.bots.ore;
    state.resources.cly += state.bots.cly;
    state.resources.obs += state.bots.obs;
    state.resources.geo += state.bots.geo;
    state.turn += 1;
}

fn buy_bot(state: &mut State, bot: u8){
    match bot{
        0 => state.bots.ore += 1,
        1 => state.bots.cly += 1,
        2 => state.bots.obs += 1,
        3 => state.bots.geo += 1,
        _ => panic!("Not a bot!")
    }
}

fn check_resources(state: &State, blueprint: &Blueprint, bot: u8, max_turns: u8)-> bool{
    if state.turn >= max_turns-1{ return false };
    if state.turn >= max_turns-2 && bot < 3 { return false };
    if state.turn >= max_turns-3 && bot < 2 { return false };
    if state.turn >= max_turns-4 && bot < 1 { return false };
    let max_ore = *[
        blueprint.ore_cost, blueprint.cly_cost, blueprint.obs_ore_cost, blueprint.geo_ore_cost
    ].iter().max().unwrap();
    match bot{
        0 => state.bots.ore < max_ore && state.resources.ore >= blueprint.ore_cost,
        1 => state.bots.cly < blueprint.obs_cly_cost && state.resources.ore >= blueprint.cly_cost,
        2 => state.resources.ore >= blueprint.obs_ore_cost && state.resources.cly >= blueprint.obs_cly_cost,
        3 => state.resources.ore >= blueprint.geo_ore_cost && state.resources.obs >= blueprint.geo_obs_cost,
        _ => panic!("Not a bot!")
    }
}

fn make_next_possible_bots(state: &State, blueprint: &Blueprint, max_turns:u8)-> Vec<State> {
    let mut out = vec![];

    for bot in 0..4{
        if check_resources(state, blueprint, bot, max_turns){
            let mut new_state = state.clone();
            spend_resources(&mut new_state, blueprint, bot);
            collect_resources(&mut new_state);
            buy_bot(&mut new_state, bot);
            out.push(new_state);
        }
    }
    // If we can build all of the bots, we should def build a bot.
    if out.len() == 4 {return out}
    let mut wait_state = state.clone();
    collect_resources(&mut wait_state);
    out.push(wait_state);
    out
}


fn do_turn(state: &State, blueprint: &Blueprint, targets: &[Robots], purchase_idx: usize, max_turns: u8) -> (State, usize){
    let mut new_state = state.clone();
    let mut out_idx = purchase_idx;
    // buy a geode robot if at all possible
    if out_idx >= targets.len(){
        spend_resources(&mut new_state, blueprint, 3);
        collect_resources(&mut new_state);
        buy_bot(&mut new_state, 3);
        return if out_idx >= targets.len(){(new_state, out_idx)}
        else if targets[out_idx] == Robots::Geo { (new_state, out_idx + 1) }
        else { (new_state, out_idx) }
    }
    // if geo buy was unsuccessful:
    // Check to see if we need to skip buying anything:
    while out_idx < targets.len(){
        if (targets[out_idx] == Robots::Ore) &
            (state.bots.ore >= *[blueprint.ore_cost, blueprint.cly_cost, blueprint.obs_ore_cost, blueprint.geo_ore_cost].iter().max().unwrap()){
            out_idx += 1;
        } else if (targets[out_idx] == Robots::Clay) & (state.bots.cly >= blueprint.obs_cly_cost){
            out_idx += 1;
        } else {break;}
    }
    if out_idx < targets.len(){
        let bot: u8;
        match targets[out_idx] {
        Robots::Ore => bot=0,
        Robots::Clay => bot=1,
        Robots::Obsidian => bot=2,
        Robots::Geo => bot=3,
        }
        while !check_resources(&new_state, blueprint, bot, max_turns) && state.turn < max_turns {
            collect_resources(&mut new_state);
        }
        if state.turn < max_turns {
            spend_resources(&mut new_state, blueprint, bot);
            collect_resources(&mut new_state);
            buy_bot(&mut new_state, bot);
            return (new_state, out_idx + 1);
        } else {
            return (new_state, out_idx + 1);
        }
    }
    // no bots to buy, so we just collect resources and increment the turn
    collect_resources(&mut new_state);
    return (new_state, purchase_idx)
}

fn run_simulation(blueprint: &Blueprint, organism: &Organism, max_turns:u8) -> u8{
    let targets = ints_to_bots(&organism.0);
    let mut state = State{..START};
    let mut purchase_idx = 0usize;
    while state.turn < 24{
        (state, purchase_idx) = do_turn(&state, blueprint, &targets, purchase_idx, max_turns);
    }
    return state.resources.geo
}

/// Runs a simulation!
fn optimize_blueprint(blueprint: &Blueprint, population_size: usize, n_generations: usize, max_turns:u8) -> (u8, HashMap<Organism, u8>){
    let mut rng = thread_rng();

    let mut outputs: HashMap<Organism, u8> = HashMap::with_capacity(population_size*n_generations);
    let mut population: Vec<Organism> = Vec::with_capacity(population_size);
    let mut scores: Vec<u8> = Vec::with_capacity(population_size);
    let mut max_score = 0u8;
    let mut generation_best = vec![];
    // generate initial population!
    for _ in 0..population_size{
        population.push(generate_organism())
    }
    population.dedup_by(|a, b| a.0==b.0);
    'outer: for generation in 0..n_generations {
        let mut local_best = 0u8;
        // check all the organisms
        for org in population.clone().into_iter() {
            let score = if outputs.contains_key(&org){
                *outputs.get(&org).unwrap()
            } else {
                let score = run_simulation(blueprint, &org, max_turns);
                outputs.insert(org.clone(), score);
                score
            };
            scores.push(score);
            max_score = max(score, max_score);
            local_best = max(score, local_best);
        }
        generation_best.push(local_best);
        if local_best == 0{
            let mut babies = vec![];
            for _ in 0..population_size*2{
                let child = generate_organism();
                if !outputs.contains_key(&child){
                    babies.push(child)
                }
            }
            babies.dedup_by(|a, b| a.0==b.0);
            population=babies;
            continue
        }
        // choose some parents!
        let samples: Vec<(&Organism, &u8)> = population.iter().zip(scores.iter()).collect();
        let weight_dist = match WeightedIndex::new(samples.clone().into_iter().map(|(_, w)| w)){
            Ok(w) => w,
            Err(_) => {
                let mut babies = vec![];
                for _ in 0..population_size*3{
                    let child = generate_organism();
                    if !outputs.contains_key(&child){
                        babies.push(child)
                    }
                }
                babies.dedup_by(|a, b| a.0==b.0);
                population=babies;
                continue 'outer;
            }
        };
        let mut parents: Vec<Organism> = vec![];
        for _ in 0..max(2, population_size/40){
            let idx = weight_dist.clone().sample(&mut rng);
            let item = samples[idx];
            parents.push(item.0.clone())
        }
        // make some babies to prepare for the next generation!!
        let mut babies = parents.clone();
        for i in 0..parents.len()-1{
            babies.extend(mate_pair(&parents[i], &parents[i+1]))
        }
        babies.extend(mutate_babies(&babies));
        babies.dedup_by(|a, b| a.0==b.0);
        babies.dedup_by(|a, b| outputs.contains_key(&a));
        'pop_increase: for n in 0..10 {
            match babies.len().cmp(&population_size) {
                Ordering::Equal => {
                    population = babies;
                    break
                },
                Ordering::Less => {
                    babies.extend(mutate_babies(&babies));
                    babies.dedup_by(|a, b| a.0==b.0);
                    babies.dedup_by(|a, b| outputs.contains_key(&a));
                    if n == 9{
                        for _ in 0..population_size-babies.clone().len() {
                            for _ in 0..100{
                                let child = generate_organism();
                                if !outputs.contains_key(&child){
                                    babies.push(child);
                                    break
                                }
                            }
                        }
                        population = babies;
                        population.extend(parents);
                        break
                    }
                },
                Ordering::Greater => {
                    babies.partial_shuffle(&mut thread_rng(), population_size);
                    babies.truncate(population_size);
                    population = babies;
                    break
                },
            };
        }
        // if max_score == 12{
        //     println!("Quitting after {generation} generations");
        //     break
        // }
        // if { generation > 3 }{
        //     if {max_score == generation_best[generation_best.len()-1]}
        //     & {max_score == generation_best[generation_best.len()-2]}
        //     & {max_score == generation_best[generation_best.len()-3]}{
        //     // let's call it quits!
        //     println!("Quitting after {generation} generations");
        //     break
        //     }
        // }

    }
    return (max_score, outputs)
}

fn explore_all_branches(
    iter_hash: &HashSet<State>,
    push_hash: &mut HashSet<State>,
    blueprint: &Blueprint,
    max_turns:u8
) -> u8{
    let mut max_geo = 0;
    for state in iter_hash.iter(){
        let p = make_next_possible_bots(state, blueprint, max_turns);
        for s in p.iter(){
            if s.resources.geo > max_geo{max_geo = s.resources.geo}
        }
        push_hash.extend(p.iter())
    }
    return max_geo
}

/*fn get_all_next_options(state: &State, blueprint: &Blueprint, bot: u8, max_turns: u8)-> Vec<u8>{
    let mut out = vec![];
    // if there aren't too many ore bots, you could always add an ore bot!
    let max_ore = *[
        blueprint.ore_cost, blueprint.cly_cost, blueprint.obs_ore_cost, blueprint.geo_ore_cost
    ].iter().max().unwrap();
    if state.bots.ore < max_ore && state.turn < max_turns-4{ out.push(0) }
    if state.bots.cly < blueprint.obs_cly_cost && state.turn < max_turns-3{ out.push(0) }
    let r_turns = max_turns - state.turn;

    if state.resources.cly + state.bots.cly * (r_turns)


    if state.turn >= max_turns-1{ return false };
    if state.turn >= max_turns-2 && bot < 3 { return false };
    if state.turn >= max_turns-3 && bot < 2 { return false };
    if state.turn >= max_turns-4 && bot < 1 { return false };

    match bot{
        0 => state.bots.ore < max_ore && state.resources.ore >= blueprint.ore_cost,
        1 => state.bots.cly < blueprint.obs_cly_cost && state.resources.ore >= blueprint.cly_cost,
        2 => state.resources.ore >= blueprint.obs_ore_cost && state.resources.cly >= blueprint.obs_cly_cost,
        3 => state.resources.ore >= blueprint.geo_ore_cost && state.resources.obs >= blueprint.geo_obs_cost,
        _ => panic!("Not a bot!")
    }
    out
}*/


// fn pick_next(state: &mut State, blueprint: &Blueprint, global_best: u8, max_turns: u8){
//     let choices = get_possible_choices(&state, blueprint: &Blueprint, max_turns);
//     for choice in choices{
//         let mut dive_state = state.clone();
//         procuess_until_bot_purchase(&mut dive_state, blueprint, max_turns, choice);
//         pick_next(&mut dive_state, blueprint, global_best, max_turns);
//     }
// }

fn tree_search(blueprint: &Blueprint, time: u8)->u8{
    let mut odd_turn_set = HashSet::new();
    odd_turn_set.insert(START);
    let mut even_turn_set = HashSet::new();
    let mut max_geo=0;
    for minute in 1..time+1{
        if minute > 26{println!("{minute}..{}", max(odd_turn_set.len(),even_turn_set.len()))};
        if minute % 2 == 1{
            max_geo = explore_all_branches(&odd_turn_set, &mut even_turn_set, blueprint, time);
            odd_turn_set.clear()
        } else{
            max_geo = explore_all_branches(&even_turn_set, &mut odd_turn_set, blueprint, time);
            even_turn_set.clear()
        }
    }
    return max_geo
}

pub(crate) fn part_1() {
    let mut blueprints = vec![];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.trim() != "" {
                blueprints.push(read_blueprint(&line));
            }
        }
    }
    let mut quality_levels: Vec<u8>= Vec::with_capacity(30);
    for b in blueprints.into_iter(){
        // let (best_score, all_scores) = optimize_blueprint(&b, 5000, 50);
        let best_score = tree_search(&b, 24);
        quality_levels.push(best_score*b.idx);
        println!("blueprint {} * {} = {}",b.idx ,best_score, best_score*b.idx);
    }
    println!("Day {DAY} Part 1: {}", quality_levels.iter().sum::<u8>()); // 1199
}

pub(crate) fn part_2() {
    let mut blueprints = vec![];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.trim() != "" {
                blueprints.push(read_blueprint(&line));
            }
        }
    }
    println!("Day {DAY} Part 2: {}", tree_search(
        &blueprints[0], 32)
        * tree_search(&blueprints[0], 32)
        * tree_search(&blueprints[0], 32)); // 3510 is correct
}
