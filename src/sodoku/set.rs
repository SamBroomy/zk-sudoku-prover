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

    pub fn is_valid(&self) -> bool {
        self.cells.iter().all_unique() && self.cells.iter().all(|&cell| cell != Cell::Empty)
    }

    pub fn is_empty(&self) -> bool {
        self.cells.iter().all(|&cell| cell == Cell::Empty)
    }

    pub fn is_partial_valid(&self) -> bool {
        self.cells
            .iter()
            .filter(|&&cell| cell != Cell::Empty)
            .all_unique()
            && !self.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_row() {
        let cells = [
            Cell::One,
            Cell::Two,
            Cell::Three,
            Cell::Four,
            Cell::Five,
            Cell::Six,
            Cell::Seven,
            Cell::Eight,
            Cell::Nine,
        ];
        let set: Set<Row> = Set::new(cells, Position::ONE);
        assert!(set.is_valid());
    }

    #[test]
    fn test_incorrect_row_with_duplicates() {
        let cells = [
            Cell::Nine,
            Cell::Two,
            Cell::Three,
            Cell::Four,
            Cell::Five,
            Cell::Six,
            Cell::Seven,
            Cell::Eight,
            Cell::Nine,
        ];
        let set: Set<Row> = Set::new(cells, Position::ONE);
        assert!(!set.is_valid());
    }

    #[test]
    fn test_incorrect_row_with_empty() {
        let cells = [
            Cell::One,
            Cell::Two,
            Cell::Three,
            Cell::Four,
            Cell::Empty,
            Cell::Six,
            Cell::Seven,
            Cell::Eight,
            Cell::Nine,
        ];
        let set: Set<Row> = Set::new(cells, Position::ONE);
        assert!(!set.is_valid());
    }

    #[test]
    fn test_correct_column() {
        let cells = [
            Cell::One,
            Cell::Two,
            Cell::Three,
            Cell::Four,
            Cell::Five,
            Cell::Six,
            Cell::Seven,
            Cell::Eight,
            Cell::Nine,
        ];
        let set: Set<Column> = Set::new(cells, Position::THREE);
        assert!(set.is_valid());
    }

    #[test]
    fn test_incorrect_column_with_empty() {
        let cells = [
            Cell::One,
            Cell::Two,
            Cell::Three,
            Cell::Four,
            Cell::Empty,
            Cell::Six,
            Cell::Seven,
            Cell::Eight,
            Cell::Nine,
        ];
        let set: Set<Column> = Set::new(cells, Position::ONE);
        assert!(!set.is_valid());
    }

    #[test]
    fn test_correct_square() {
        let cells = [
            Cell::One,
            Cell::Five,
            Cell::Three,
            Cell::Four,
            Cell::Nine,
            Cell::Two,
            Cell::Six,
            Cell::Seven,
            Cell::Eight,
        ];
        let set: Set<Box> = Set::new(cells, Position::ONE);
        assert!(set.is_valid());
    }

    #[test]
    fn test_getters() {
        let cells = [Cell::One; 9];
        let position = Position::FIVE;
        let set: Set<Row> = Set::new(cells, position);

        assert_eq!(set.position(), position);
        assert_eq!(set.cells(), &cells);
    }

    #[test]
    fn test_set_type() {
        assert_eq!(Row::get_type(), "Row");
        assert_eq!(Column::get_type(), "Column");
        assert_eq!(Box::get_type(), "Square");
    }
}
