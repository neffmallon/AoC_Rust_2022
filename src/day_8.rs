use crate::general_helpers::read_day_input_lines;


const DAY: u8 = 8;
// problem at https://adventofcode.com/2022/day/8
// Trees!!!!

fn get_forest_size()->(usize, usize){
    let mut n_lines = 0usize;
    let mut n_cols = 0usize;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            if line.trim() != ""{n_lines += 1}
            if n_cols == 0{ n_cols = line.trim().len() }
        }
    }
    return (n_lines, n_cols)
}

pub(crate) fn part_1() {
    let (n_lines, n_cols) = get_forest_size();
    let mut forest = vec![vec![0i16;n_cols];n_lines];
    let mut visible = vec![vec![0u32;n_cols];n_lines];
    // read in forest values:
    if let Ok(lines) = read_day_input_lines(DAY) {
        let mut row_num = 0;
        for line in lines.flatten() {
            forest[row_num] = line.trim().chars().map(|c| c.to_digit(10).unwrap() as i16).collect();
            row_num+=1;
        }
    }
    let frozen_forest = &forest;
    let mut tallest_col_trees = vec![-1i16;n_cols];
    // check the trees for visibility!
    for (row_idx, row) in frozen_forest.iter().enumerate(){
        let mut tallest_tree = -1i16;
        for (col_idx, tree) in row.iter().enumerate(){
            if *tree > tallest_tree{
                visible[row_idx][col_idx] = 1;
                tallest_tree = *tree;
            }
            if tree > &tallest_col_trees[col_idx]{
                visible[row_idx][col_idx] = 1;
                tallest_col_trees[col_idx] = tree.clone();
            }
        }
    }
    tallest_col_trees = vec![-1i16;n_cols];
    // check the trees for visibility!
    for (row_idx, row) in frozen_forest.iter().rev().enumerate(){
        let mut tallest_tree = -1i16;
        for (col_idx, tree) in row.iter().rev().enumerate(){
            let new_row_idx = n_lines - row_idx - 1;
            let new_col_idx = n_cols - col_idx - 1;
            if *tree > tallest_tree{
                visible[new_row_idx][new_col_idx] = 1;
                tallest_tree = *tree;
            }
            if tree > &tallest_col_trees[new_col_idx]{
                visible[new_row_idx][new_col_idx] = 1;
                tallest_col_trees[new_col_idx] = tree.clone();
            }
        }
    }
    let visible_count: u32 = visible.iter().map(|v| v.iter().sum::<u32>()).collect::<Vec<u32>>().iter().sum();
    println!("Day {DAY} part 1: Total Visible Trees: {:?}",visible_count);
}

fn compute_score(forest: &Vec<Vec<i16>>, row: usize, col: usize)-> u64{
    let n_lines = forest.len();
    let n_cols = forest[0].len();
    if { ( n_cols == col + 1 ) || ( n_lines == row + 1 ) || ( col ==0 ) || (row==0)}{ return 0}
    let mut up_score = 0u64;
    let mut down_score = 0u64;
    let mut right_score = 0u64;
    let mut left_score = 0u64;
    // up score:
    for i in 1..(row+1){
        up_score += 1;
        if forest[row-i][col] >= forest[row][col]{
            break;
        }
    }
    // down score:
    for i in (row+1)..n_lines{
        down_score += 1;
        if forest[i][col] >= forest[row][col]{
            break;
        }
    }
    // left score:
    for i in 1..(col+1) {
        left_score += 1;
        if forest[row][col - i] >= forest[row][col] {
            break;
        }
    }
    // right score:
    for i in (col+1)..n_cols{
        right_score += 1;
        if forest[row][i] >= forest[row][col]{
            break;
        }
    }
    return up_score * down_score * left_score * right_score

}
pub(crate) fn part_2() {
    let (n_lines, n_cols) = get_forest_size();
    let mut forest = vec![vec![0i16;n_cols];n_lines];
    let mut scores = vec![vec![0u64;n_cols];n_lines];
    // read in forest values:
    if let Ok(lines) = read_day_input_lines(DAY) {
        let mut row_num = 0;
        for line in lines.flatten() {
            forest[row_num] = line.trim().chars().map(|c| c.to_digit(10).unwrap() as i16).collect();
            row_num+=1;
        }
    }
    for i in 1..(n_lines-1){
        for j in 1..(n_cols-1){
            scores[i][j] = compute_score(&forest, i, j)
        }
    }
    let high_score = scores.iter()
        .map(|v| v.iter().max().unwrap().clone())
        .collect::<Vec<u64>>()
        .iter().max().unwrap().clone();
    println!("Day {DAY} part 2: High Score: {:?}",high_score);
}