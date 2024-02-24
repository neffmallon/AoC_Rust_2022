use std::cmp::{Ordering};
use std::collections::{HashMap, HashSet};

use crate::general_helpers::read_day_input_lines;
use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;

const DAY: u8 = 16;
// problem at https://adventofcode.com/2022/day/16
// It's the classic Traveling Volcano Plumber problem!

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Room{
    name: String,
    flow: u32,
}

fn parse_line(line: &str)-> (String, u32, Vec<String>){
    let s = line.split(';').collect::<Vec<&str>>();
    let flow: u32 = s[0].split('=').collect::<Vec<&str>>()[1].parse().unwrap();
    let name: String = s[0].split(' ').collect::<Vec<&str>>()[1].to_string();
    // plurals are annoying
    if s[1].contains("valves "){
        let tunnels: Vec<String> = s[1].split("valves ").collect::<Vec<&str>>()[1]
        .split(", ").map(|string| string.to_string()).collect();
        (name, flow, tunnels)
    } else{
            let tunnels: Vec<String> = s[1].split("valve ").collect::<Vec<&str>>()[1]
        .split(", ").map(|string| string.to_string()).collect();
        (name, flow, tunnels)
    }
}

fn read_graph()->(Vec<Room>, Vec<(String, String, f32)>) {
    let mut room_array: Vec<Room> = vec![];
    let mut path_hashset: HashSet<(String, String)> = HashSet::new();

    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            let (name, flow, connections) = parse_line(line.trim());
            room_array.push(Room{name: name.clone(), flow});
            for c in connections.into_iter(){
                match name.cmp(&c){
                    Ordering::Greater => path_hashset.insert((c.clone(), name.clone())),
                    Ordering::Less => path_hashset.insert((name.clone(), c.clone())),
                    Ordering::Equal => panic!{"A room cannot connect with itself!"}
                };
            }
        }
    }

    let mut path_array: Vec<(String, String, f32)> = vec![];
    for p in path_hashset.into_iter(){
        path_array.push((p.0,p.1,1.0))
    }
    (room_array, path_array)
}

fn make_graph_from_rooms_and_edges(
    rooms: &[Room], edges: &[(String, String, f32)]
)-> (Graph<Room,f32, Undirected>, HashMap<String, NodeIndex>) {
    let mut idx_hash: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph:Graph<Room,f32, Undirected> = Graph::new_undirected();
    for room in rooms.iter(){
        idx_hash.insert(room.name.clone(), graph.add_node(room.clone()));
    }
    // Create Edges!
    for path in edges.iter(){
        graph.add_edge(*idx_hash.get(&path.0).unwrap(), *idx_hash.get(&path.1).unwrap(), path.2);
    }
    (graph, idx_hash)
}

/// Makes a copy of a the tunnel system graph that connects the starting room and all other rooms
/// directly to each other, including the time it takes to turn on a valve at the destination.
/// The reduced graph then removes all nodes that have 0 flow and all edges that have a weight of 1.
fn make_condensed_graph(graph: &Graph<Room,f32, Undirected>) ->Graph<Room,f32, Undirected>{
    let mut out_graph = graph.clone();
    // find all the new distances! Add 1 to account for the time it takes to turn the faucet on
    for node in out_graph.node_indices(){
        let result = petgraph::algo::dijkstra(graph, node, None, |e| *e.weight());
        // Distances are being found on the base graph, so modifying the edges here doesn't matter
        for (to_node, dist) in result.into_iter() {
            out_graph.update_edge(
                node, to_node, dist+1.0
            );
        }
    }
    // clear out the out nodes that we have eliminated
    // Because any removal invalidates indices, we have to restart the for loop on each removal
    let mut keep_removing = true;
    while keep_removing {
        keep_removing = false;
        for node in out_graph.node_indices() {
            let node_weight = match out_graph.node_weight(node){
                None => panic!("Node index {node:?} not found"),
                Some(n) => n,
            };
            if node_weight.name == "AA" { continue }
            else if node_weight.flow == 0 {
                out_graph.remove_node(node);
                keep_removing = true;
                break
            } else { continue };
        }
    }
    keep_removing = true;
    while keep_removing {
        keep_removing = false;
        for edge_idx in out_graph.edge_indices() {
            let edge_weight = match out_graph.edge_weight(edge_idx){
                None => panic!("Edge index {edge_idx:?} not found"),
                Some(n) => n,
            };
            if edge_weight == &1.0f32 {
                out_graph.remove_edge(edge_idx);
                keep_removing = true;
                break
            } else { continue };
        }
    }

    out_graph
}

fn find_node_idx(room_name: &str, graph: &Graph<Room,f32, Undirected>) -> Option<NodeIndex> {
    for node_idx in graph.node_indices() {
        let node_weight = match graph.node_weight(node_idx) {
            None => panic!("Node index {node_idx:?} not found"),
            Some(n) => n,
        };
        if node_weight.name != room_name { continue }
        return Some(node_idx)
    };
    None
}

fn take_all_next_steps_hashset(
    all_paths: &mut Vec<(HashSet<NodeIndex>, NodeIndex, u32 , u32)>,
    current_idx: usize,
    max_profit: (HashSet<NodeIndex>, u32),
    graph: &Graph<Room,f32, Undirected>
)-> (HashSet<NodeIndex>, u32){
    let mut current_max_profit = max_profit.1;
    let mut current_max_visits = max_profit.0;
    let current_state: (HashSet<NodeIndex>, NodeIndex, u32 , u32) = all_paths[current_idx].clone();
    if current_state.3 < 2{return (current_max_visits, current_max_profit)}
    // Add all potential next nodes
    for node_idx in graph.node_indices(){
        let node_weight: &Room = graph.node_weight(node_idx).unwrap();
        // we are only going to visit each node once, so if we find a duplicate, keep going
        if node_weight.flow == 0{continue}
        if current_state.0.contains(&node_idx) {continue}
        let edge_weight: &f32 = graph.edge_weight(
            graph.find_edge(current_state.1, node_idx).unwrap()
        ).unwrap();
        // If the next node is too far away, keep checking
        if current_state.3 < (*edge_weight as u32) {continue}
        else{
            let remaining_time = current_state.3 - (*edge_weight as u32);
            let node_weight: &Room = graph.node_weight(node_idx).unwrap();
            let mut new_visited = current_state.0.clone();
            new_visited.insert(node_idx);
            let new_score = remaining_time * node_weight.flow + current_state.2;
            all_paths.push(
                (
                    new_visited.clone(), node_idx, new_score, remaining_time,
                )
            );
            if new_score > current_max_profit{
                current_max_profit = new_score;
                current_max_visits = new_visited;
            }
        }
    }
    (current_max_visits, current_max_profit)
}

pub(crate) fn part_1() {
    let (room_array, path_array) = read_graph();
    let (full_graph, _full_idx_lookup) = make_graph_from_rooms_and_edges(&room_array, &path_array);
    let graph = make_condensed_graph(&full_graph);
    let mut all_states: Vec<(HashSet<NodeIndex>, NodeIndex, u32 , u32)> = vec![
        (HashSet::new(), find_node_idx("AA", &graph).unwrap(), 0, 30)
    ];
    let mut current_path_idx = 0usize;
    let mut current_max_profit = (all_states[0].0.clone(), 0u32);
    // we find the best path for a single agent in 26 minutes.
    while current_path_idx < all_states.len(){
        current_max_profit = take_all_next_steps_hashset(
            &mut all_states, current_path_idx, current_max_profit, &graph
        );
        current_path_idx +=1;
    }

    println!("length of all paths: {:?}",all_states.len());
    println!("Day {DAY} Part 1: {}",current_max_profit.1);
}

pub(crate) fn part_2() {
    let (room_array, path_array) = read_graph();
    let (full_graph, _full_idx_lookup) = make_graph_from_rooms_and_edges(&room_array, &path_array);
    let graph = make_condensed_graph(&full_graph);
    let mut all_states: Vec<(HashSet<NodeIndex>, NodeIndex, u32 , u32)> = vec![
        (HashSet::new(), find_node_idx("AA", &graph).unwrap(), 0, 26)
    ];
    let mut current_path_idx = 0usize;
    let mut current_max_profit = (all_states[0].0.clone(), 0u32);
    // we find the best path for a single agent in 26 minutes.
    while current_path_idx < all_states.len(){
        current_max_profit = take_all_next_steps_hashset(
            &mut all_states, current_path_idx, current_max_profit, &graph
        );
        current_path_idx +=1;
    }
    // Now we find the best path to complement the best single agent
    let mut best_elephant_state_score: u32 = 0;
    let mut current_elephant_idx = 0usize;
    while current_elephant_idx < all_states.len(){
        if best_elephant_state_score < all_states[current_elephant_idx].2 && current_max_profit.0.is_disjoint(&all_states[current_elephant_idx].0) {
            best_elephant_state_score = all_states[current_elephant_idx].2;
        }
        current_elephant_idx += 1
    }
    let mut current_best_team = current_max_profit.1 + best_elephant_state_score;
    println!("length of all states: {}",all_states.len());
    println!("Max profit: {}, elephant profit baseline {}",current_max_profit.1, best_elephant_state_score);
    println!("current best team {}", current_best_team);
    let mut states_to_consider = vec![];
    for state in all_states.into_iter(){
        if state.2 > best_elephant_state_score{
            states_to_consider.push(state)
        }
    }
    println!("Number of states to consider: {}", states_to_consider.len());
    states_to_consider.sort_by(|a, b| a.2.cmp(&b.2));
    let s = states_to_consider.clone();
    for agent1 in states_to_consider.into_iter(){
        for agent2 in s.clone().into_iter().rev(){
            if agent1.2 > agent2.2{break}
            if agent1.2 + agent2.2 < current_best_team{break}
            if agent1.0.is_disjoint(&agent2.0){
                current_best_team = agent1.2 + agent2.2;
                break
            }
        }
    }
    println!("Day {DAY} Part 2: {current_best_team}"); //2999
}
