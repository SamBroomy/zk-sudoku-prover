use std::{fmt, str::FromStr};

use super::{Box, Cell, Column, Position, Row, Set};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SudokuGrid {
    cells: [[Cell; 9]; 9],
}

impl SudokuGrid {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Empty; 9]; 9],
        }
    }

    pub fn get_row(&self, row: Position) -> Set<Row> {
        Set::new(self.cells[row], row)
    }

    pub fn get_column(&self, col: Position) -> Set<Column> {
        let mut new_col = [Cell::Empty; 9];
        for row in Position::ALL_POSITIONS {
            new_col[row] = self.cells[row][col];
        }
        Set::new(new_col, col)
    }

    pub fn get_square(&self, pos: Position) -> Set<Box> {
        let mut new_square = [Cell::Empty; 9];
        let points = pos.get_box_positions();
        for (i, point) in points.into_iter().enumerate() {
            new_square[i] = self.cells[point];
        }
        Set::new(new_square, pos)
    }

    pub fn is_valid_solution(&self) -> bool {
        for row in Position::ALL_POSITIONS {
            if !self.get_row(row).is_valid() {
                return false;
            }
        }
        for col in Position::ALL_POSITIONS {
            if !self.get_column(col).is_valid() {
                return false;
            }
        }
        for square in Position::ALL_POSITIONS {
            if !self.get_square(square).is_valid() {
                return false;
            }
        }
        true
    }
}

impl FromStr for SudokuGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 81 {
            return Err(());
        }
        let mut cells = [[Cell::Empty; 9]; 9];
        for (i, c) in s.chars().enumerate() {
            cells[i / 9][i % 9] = Cell::from(c);
        }
        Ok(Self { cells })
    }
}

impl fmt::Display for SudokuGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..9 {
            for col in 0..9 {
                write!(f, "{}", self.cells[row][col])?;
                if col % 3 == 2 && col != 8 {
                    write!(f, "|")?;
                }
            }
            writeln!(f)?;
            if row % 3 == 2 && row != 8 {
                writeln!(f, "---+---+---")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../../data/validation.csv");

    fn parse_input() -> Vec<SudokuGrid> {
        INPUT
            .lines()
            .filter_map(|l| {
                let line = l.trim();
                if l.is_empty() {
                    None
                } else {
                    SudokuGrid::from_str(line).ok()
                }
            })
            .collect::<Vec<_>>()
    }
}
