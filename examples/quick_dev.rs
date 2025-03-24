use std::str::FromStr;

use petgraph::dot::{Config, Dot};
use zk_sudoku_prover::*;

fn main() {
    let input = include_str!("../data/validation.csv");
    println!("Input: {}", input);
    let line = input.lines().next().unwrap();
    let board = SudokuGrid::from_str(line).unwrap();
    println!("Board:\n{}", board);
    println!("Valid: {}", board.is_valid_solution());
    let graph = Graph::from_sudoku(&board);
    let g = graph.graph;

    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}
