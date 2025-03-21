use std::str::FromStr;

use zk_sudoku_prover::*;

fn main() {
    let input = include_str!("../data/validation.csv");
    println!("Input: {}", input);
    let line = input.lines().next().unwrap();
    let board = SudokuGrid::from_str(line).unwrap();
    println!("Board:\n{}", board);
    println!("Valid: {}", board.is_valid_solution());
}
