use std::cmp::{max, Ordering};
use regex::Regex;

use std::collections::HashMap;
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

const GENOME_SIZE: usize = 13;

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
    idx: u32,
    ore_cost: u32, // What an ore robot costs in ore
    cly_cost: u32, // what a clay robot costs in ore
    obs_ore_cost: u32, // what an obsidian robot costs in ore
    obs_cly_cost: u32, // what an obsidian robot costs in clay
    geo_ore_cost: u32, // what a geode robot costs in ore
    geo_obs_cost: u32, // what a geode robot costs in obsidian
}
#[derive(PartialEq)]
enum Robots{
    Ore,
    Clay,
    Obsidian,
    Geo,
}

#[derive(Copy, Clone, Debug)]
struct State{
    turn: u32,
    ore: u32,
    cly: u32,
    obs: u32,
    geo: u32,
    r_ore: u32,
    r_cly: u32,
    r_obs: u32,
    r_geo: u32,
}

const START: State = State{
    turn: 0,
    ore: 0,
    cly: 0,
    obs: 0,
    geo: 0,
    r_ore: 1,
    r_cly: 0,
    r_obs: 0,
    r_geo: 0,
};

fn read_blueprint(s: &str)-> Blueprint{
    let re = Regex::new(r"\d+").unwrap();
    let mut caps: Vec<u32> = vec![];
    for (num, []) in re.captures_iter(s).map(|c| c.extract()) {
        caps.push(num.parse::<u32>().unwrap());
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

fn collect_resources(state: &mut State){
    state.geo += state.r_geo;
    state.cly += state.r_cly;
    state.ore += state.r_ore;
    state.obs += state.r_obs;
}

fn buy_bots(state: &mut State, bot_vec: &[u32;4]){
    state.r_ore += bot_vec[0];
    state.r_cly += bot_vec[1];
    state.r_obs += bot_vec[2];
    state.r_geo += bot_vec[3];
}

fn do_turn(state: &State, blueprint: &Blueprint, targets: &[Robots], purchase_idx: usize) -> (State, usize){
    let mut bots_to_add = [0u32;4];
    let mut new_state = state.clone();
    let mut out_idx = purchase_idx;
    // buy a geode robot if at all possible
    if (new_state.ore >= blueprint.geo_ore_cost) & (new_state.obs >= blueprint.geo_obs_cost){
        bots_to_add[3] += 1;
        new_state.ore -= blueprint.geo_ore_cost;
        new_state.obs -= blueprint.geo_obs_cost;
    } else {
        while out_idx < targets.len(){
            if (targets[out_idx] == Robots::Ore) &
                (state.r_ore > max(max(max(blueprint.ore_cost, blueprint.cly_cost), blueprint.obs_ore_cost), blueprint.geo_ore_cost)){
                out_idx += 1;
            } else if (targets[out_idx] == Robots::Clay) & (state.r_cly >blueprint.obs_cly_cost){
                out_idx += 1;
            } else {break;}
        }
        // we have already tried to buy a geode robot, so we only have anything left to try if
        // there are still targets left.
        if out_idx < targets.len(){
                match targets[purchase_idx] {
                Robots::Ore => if new_state.ore >= blueprint.ore_cost {
                    bots_to_add[0] += 1;
                    new_state.ore -= blueprint.ore_cost;
                },
                Robots::Clay => if new_state.ore >= blueprint.cly_cost {
                    bots_to_add[1] += 1;
                    new_state.ore -= blueprint.cly_cost;
                },
                Robots::Obsidian => if (new_state.ore >= blueprint.obs_ore_cost)
                    & (new_state.cly >= blueprint.obs_cly_cost) {
                    bots_to_add[2] += 1;
                    new_state.ore -= blueprint.obs_ore_cost;
                    new_state.cly -= blueprint.obs_cly_cost;
                },
                Robots::Geo => (),
            }
        }
    }
    let purchase_count = (bots_to_add[0]+bots_to_add[1]+bots_to_add[2]+bots_to_add[3]) as usize;
    collect_resources(&mut new_state);
    buy_bots(&mut new_state, &bots_to_add);
    new_state.turn += 1;
    return (new_state, purchase_idx+purchase_count)
}

fn run_simulation(blueprint: &Blueprint, organism: &Organism) -> u32{
    let targets = ints_to_bots(&organism.0);
    let mut state = State{..START};
    let mut purchase_idx = 0usize;
    while state.turn < 24{
        (state, purchase_idx) = do_turn(&state, blueprint, &targets, purchase_idx);
    }
    return state.geo
}

/// Runs a simulation!
fn optimize_blueprint(blueprint: &Blueprint, population_size: usize, n_generations: usize) -> (u32, HashMap<Organism, u32>){
    let mut rng = thread_rng();

    let mut outputs: HashMap<Organism, u32> = HashMap::with_capacity(population_size*n_generations);
    let mut population: Vec<Organism> = Vec::with_capacity(population_size);
    let mut scores: Vec<u32> = Vec::with_capacity(population_size);
    let mut max_score = 0u32;
    let mut generation_best = vec![];
    // generate initial population!
    for _ in 0..population_size{
        population.push(generate_organism())
    }
    population.dedup_by(|a, b| a.0==b.0);
    'outer: for generation in 0..n_generations {
        let mut local_best = 0u32;
        // check all the organisms
        for org in population.clone().into_iter() {
            let score = if outputs.contains_key(&org){
                *outputs.get(&org).unwrap()
            } else {
                let score = run_simulation(blueprint, &org);
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
        let samples: Vec<(&Organism, &u32)> = population.iter().zip(scores.iter()).collect();
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


pub(crate) fn part_1() {
    let mut blueprints = vec![];
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.trim() != "" {
                blueprints.push(read_blueprint(&line));
            }
        }
    }
    // let mut state = State{..START};
    // let int_targets: &[u8] = &[0,1,1,1,2,2,2,2,3,2,3,2,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3];
    // let targets = ints_to_bots(int_targets);
    // let mut purchase_idx = 0usize;
    //
    // println!("{:?}",blueprints[0]);
    // while state.turn < 24{
    //     (state, purchase_idx) = do_turn(&state, &blueprints[0], &targets, purchase_idx);
    //     println!("{:?}",state)
    // }
    let mut quality_levels: Vec<u32>= Vec::with_capacity(30);
    for b in blueprints.into_iter(){
        let (best_score, all_scores) = optimize_blueprint(&b, 5000, 50);
        quality_levels.push(best_score*b.idx);
        println!("Day {DAY} blueprint {}: {}",b.idx ,best_score);
    }
    println!("Day {DAY} Part 1: {}", quality_levels.iter().sum::<u32>()); // 1054, 1067, 1101 is too low // 1114 is also wrong
}

pub(crate) fn part_2() {
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
        }
    }
    println!("Day {DAY} Part 2: incomplete");
}
