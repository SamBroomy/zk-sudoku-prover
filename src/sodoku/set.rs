use std::marker::PhantomData;

use itertools::Itertools;

use super::{cell::Cell, position::Position};

pub struct Row;
pub struct Column;
pub struct Box;
pub trait SetType {
    fn get_type() -> &'static str {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("Unknown")
    }
}
impl SetType for Row {}
impl SetType for Column {}
impl SetType for Box {}

pub struct Set<T: SetType> {
    cells: [Cell; 9],
    position: Position,
    set_type: PhantomData<T>,
}

impl<T: SetType> Set<T> {
    pub fn new(cells: [Cell; 9], position: Position) -> Self {
        Self {
            cells,
            position,
            set_type: PhantomData,
        }
    }

    pub fn cells(&self) -> &[Cell; 9] {
        &self.cells
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn is_complete(&self) -> bool {
        if !self.is_filled() {
            return false;
        }

        // Get the values as a vec and check they're all unique
        let values: Vec<_> = self.cells.iter().filter_map(|cell| cell.value()).collect();

        if values.len() != 9 {
            return false;
        }

        // Check that all values are unique
        values.iter().all_unique()
    }

    /// Checks if the set is valid so far - no duplicate values
    /// (but may contain empties or be incomplete)
    pub fn is_valid(&self) -> bool {
        // Only check non-empty cells for uniqueness
        self.cells
            .iter()
            .filter_map(|cell| cell.value())
            .all_unique()
    }

    /// Checks if all cells are empty
    pub fn is_empty(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_empty())
    }

    /// Checks if all cells are filled (no empties)
    pub fn is_filled(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_filled())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_row() {
        let cells = [
            Cell::new_guess(1),
            Cell::new_guess(2),
            Cell::new_guess(3),
            Cell::new_guess(4),
            Cell::new_hint(5),
            Cell::new_guess(6),
            Cell::new_guess(7),
            Cell::new_guess(8),
            Cell::new_guess(9),
        ];
        let set: Set<Row> = Set::new(cells, Position::ONE);
        assert!(set.is_valid());
    }

    #[test]
    fn test_incorrect_row_with_duplicates() {
        let cells = [
            Cell::new_guess(9),
            Cell::new_guess(2),
            Cell::new_guess(3),
            Cell::new_guess(4),
            Cell::new_guess(5),
            Cell::new_hint(6),
            Cell::new_guess(7),
            Cell::new_guess(8),
            Cell::new_guess(9),
        ];

        let set: Set<Row> = Set::new(cells, Position::ONE);
        assert!(!set.is_valid());
    }

    #[test]
    fn test_incorrect_row_with_empty() {
        let cells = [
            Cell::new_guess(1),
            Cell::new_guess(2),
            Cell::new_guess(3),
            Cell::new_guess(4),
            Cell::new_hint(5),
            Cell::new_empty(),
            Cell::new_guess(7),
            Cell::new_guess(8),
            Cell::new_guess(9),
        ];
        let set: Set<Row> = Set::new(cells, Position::ONE);
        assert!(set.is_valid());
    }

    #[test]
    fn test_correct_column() {
        let cells = [
            Cell::new_guess(1),
            Cell::new_guess(2),
            Cell::new_guess(3),
            Cell::new_guess(4),
            Cell::new_hint(5),
            Cell::new_guess(6),
            Cell::new_guess(7),
            Cell::new_guess(8),
            Cell::new_guess(9),
        ];
        let set: Set<Column> = Set::new(cells, Position::THREE);
        assert!(set.is_valid());
    }

    #[test]
    fn test_incorrect_column_with_empty() {
        let cells = [
            Cell::new_guess(1),
            Cell::new_guess(2),
            Cell::new_guess(3),
            Cell::new_guess(4),
            Cell::new_empty(),
            Cell::new_guess(6),
            Cell::new_guess(7),
            Cell::new_guess(8),
            Cell::new_guess(9),
        ];
        let set: Set<Column> = Set::new(cells, Position::ONE);
        assert!(set.is_valid());
    }

    #[test]
    fn test_correct_square() {
        let cells = [
            Cell::new_guess(1),
            Cell::new_guess(5),
            Cell::new_guess(3),
            Cell::new_guess(4),
            Cell::new_hint(9),
            Cell::new_guess(2),
            Cell::new_guess(6),
            Cell::new_guess(7),
            Cell::new_guess(8),
        ];
        let set: Set<Box> = Set::new(cells, Position::ONE);
        assert!(set.is_valid());
    }

    #[test]
    fn test_getters() {
        let cells = [Cell::new_guess(1); 9];
        let position = Position::FIVE;
        let set: Set<Row> = Set::new(cells, position);

        assert_eq!(set.position(), position);
        assert_eq!(set.cells(), &cells);
    }

    #[test]
    fn test_set_type() {
        assert_eq!(Row::get_type(), "Row");
        assert_eq!(Column::get_type(), "Column");
        assert_eq!(Box::get_type(), "Box");
    }
}
