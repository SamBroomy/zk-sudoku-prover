use std::ops::{Index, IndexMut};

use super::{Point, cell::Cell};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Position {
    #[default]
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
}

impl Position {
    pub const ALL_POSITIONS: [Position; 9] = [
        Position::ONE,
        Position::TWO,
        Position::THREE,
        Position::FOUR,
        Position::FIVE,
        Position::SIX,
        Position::SEVEN,
        Position::EIGHT,
        Position::NINE,
    ];

    fn random() -> Position {
        use rand::Rng;
        let mut rng = rand::rng();
        let index = rng.random_range(0..9);
        Position::from_index(index)
    }

    fn to_index(self) -> usize {
        match self {
            Position::ONE => 0,
            Position::TWO => 1,
            Position::THREE => 2,
            Position::FOUR => 3,
            Position::FIVE => 4,
            Position::SIX => 5,
            Position::SEVEN => 6,
            Position::EIGHT => 7,
            Position::NINE => 8,
        }
    }

    fn from_index(index: usize) -> Position {
        match index {
            0 => Position::ONE,
            1 => Position::TWO,
            2 => Position::THREE,
            3 => Position::FOUR,
            4 => Position::FIVE,
            5 => Position::SIX,
            6 => Position::SEVEN,
            7 => Position::EIGHT,
            8 => Position::NINE,
            _ => panic!("Invalid index for position: {}", index),
        }
    }

    /// Returns an iterator over all the positions on the board.
    pub fn all_board_positions() -> impl Iterator<Item = Point> {
        itertools::iproduct!(Self::ALL_POSITIONS, Self::ALL_POSITIONS)
            .map(|(x, y)| Point::new(x, y))
    }

    /// Returns the positions of the cells in the row that contains this position.
    pub fn get_row_positions(&self) -> [Point; 9] {
        let mut positions = [Point::default(); 9];
        for (i, pos) in Self::ALL_POSITIONS.iter().enumerate() {
            positions[i] = Point::new(*pos, *self);
        }
        positions
    }

    /// Returns the positions of the cells in the column that contains this position.
    pub fn get_column_positions(&self) -> [Point; 9] {
        let mut positions = [Point::default(); 9];
        for (i, pos) in Self::ALL_POSITIONS.iter().enumerate() {
            positions[i] = Point::new(*self, *pos);
        }
        positions
    }

    /// Returns the positions of the cells in the box that contains this position.
    pub fn get_box_positions(&self) -> [Point; 9] {
        let start_x = (self.to_index() % 3) * 3;
        let start_y = (self.to_index() / 3) * 3;
        let mut positions = [Point::default(); 9];
        for (i, (x, y)) in itertools::iproduct!(0..3, 0..3).enumerate() {
            positions[i] = Point::new(
                Position::from_index(start_x + x),
                Position::from_index(start_y + y),
            );
        }
        positions
    }
}

impl TryFrom<u8> for Position {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Position::ONE),
            2 => Ok(Position::TWO),
            3 => Ok(Position::THREE),
            4 => Ok(Position::FOUR),
            5 => Ok(Position::FIVE),
            6 => Ok(Position::SIX),
            7 => Ok(Position::SEVEN),
            8 => Ok(Position::EIGHT),
            9 => Ok(Position::NINE),
            _ => Err(format!("Invalid position value: {}", value)),
        }
    }
}

impl Index<Position> for [Cell; 9] {
    type Output = Cell;
    fn index(&self, index: Position) -> &Self::Output {
        unsafe {
            // SAFETY: The index is guaranteed to be in the range 0..9
            // because the Position enum only has 9 variants.
            self.get_unchecked(index.to_index())
        }
    }
}

impl IndexMut<Position> for [Cell; 9] {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        unsafe {
            // SAFETY: The index is guaranteed to be in the range 0..9
            // because the Position enum only has 9 variants.
            self.get_unchecked_mut(index.to_index())
        }
    }
}

impl Index<Position> for [[Cell; 9]; 9] {
    type Output = [Cell; 9];

    fn index(&self, index: Position) -> &Self::Output {
        unsafe {
            // SAFETY: The index is guaranteed to be in the range 0..9
            // because the Position enum only has 9 variants.
            self.get_unchecked(index.to_index())
        }
    }
}

impl IndexMut<Position> for [[Cell; 9]; 9] {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        unsafe {
            // SAFETY: The index is guaranteed to be in the range 0..9
            // because the Position enum only has 9 variants.
            self.get_unchecked_mut(index.to_index())
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_get_row_position() {
        let pos = Position::random();

        let row_positions = pos.get_row_positions();
        for (i, point) in row_positions.iter().enumerate() {
            assert_eq!(point.y(), pos);
            assert_eq!(point.x(), Position::from_index(i));
        }
        assert_eq!(row_positions.len(), 9);
    }

    #[test]
    fn test_get_column_position() {
        let pos = Position::random();

        let col_positions = pos.get_column_positions();
        for (i, point) in col_positions.iter().enumerate() {
            assert_eq!(point.x(), pos);
            assert_eq!(point.y(), Position::from_index(i));
        }
        assert_eq!(col_positions.len(), 9);
    }

    #[test]
    fn test_get_square_position() {
        let pos = Position::random();
        // Calculate the expected square boundaries
        let square_x_start = (pos.to_index() % 3) * 3;
        let square_y_start = (pos.to_index() / 3) * 3;
        let square_positions = pos.get_box_positions();

        let mut seen_positions = std::collections::HashSet::new();
        for point in square_positions.iter() {
            println!("{:?}", point);

            // Check position is within the square
            assert!(
                point.x().to_index() >= square_x_start && point.x().to_index() < square_x_start + 3,
                "x coordinate outside of expected square range"
            );
            assert!(
                point.y().to_index() >= square_y_start && point.y().to_index() < square_y_start + 3,
                "y coordinate outside of expected square range"
            );

            // Add to seen positions
            seen_positions.insert((point.x().to_index(), point.y().to_index()));
        }

        // Check that all positions in the square are present (no duplicates)
        assert_eq!(
            seen_positions.len(),
            9,
            "Not all unique positions in the 3x3 grid are present"
        );
    }

    #[test]
    fn test_to_index() {
        assert_eq!(Position::ONE.to_index(), 0);
        assert_eq!(Position::TWO.to_index(), 1);
        assert_eq!(Position::THREE.to_index(), 2);
        assert_eq!(Position::FOUR.to_index(), 3);
        assert_eq!(Position::FIVE.to_index(), 4);
        assert_eq!(Position::SIX.to_index(), 5);
        assert_eq!(Position::SEVEN.to_index(), 6);
        assert_eq!(Position::EIGHT.to_index(), 7);
        assert_eq!(Position::NINE.to_index(), 8);
    }

    #[test]
    fn test_from_index() {
        assert_eq!(Position::from_index(0), Position::ONE);
        assert_eq!(Position::from_index(1), Position::TWO);
        assert_eq!(Position::from_index(2), Position::THREE);
        assert_eq!(Position::from_index(3), Position::FOUR);
        assert_eq!(Position::from_index(4), Position::FIVE);
        assert_eq!(Position::from_index(5), Position::SIX);
        assert_eq!(Position::from_index(6), Position::SEVEN);
        assert_eq!(Position::from_index(7), Position::EIGHT);
        assert_eq!(Position::from_index(8), Position::NINE);
    }

    #[test]
    #[should_panic(expected = "Invalid index for position: 9")]
    fn test_from_index_invalid() {
        Position::from_index(9);
    }

    #[test]
    fn test_try_from_u8() {
        assert_eq!(Position::try_from(1u8).unwrap(), Position::ONE);
        assert_eq!(Position::try_from(5u8).unwrap(), Position::FIVE);
        assert_eq!(Position::try_from(9u8).unwrap(), Position::NINE);

        assert!(Position::try_from(0u8).is_err());
        assert!(Position::try_from(10u8).is_err());
    }

    #[test]
    fn test_all_positions_constant() {
        assert_eq!(Position::ALL_POSITIONS.len(), 9);
        for (i, pos) in Position::ALL_POSITIONS.iter().enumerate() {
            assert_eq!(pos.to_index(), i);
        }
    }

    #[test]
    fn test_all_board_positions() {
        let positions: Vec<Point> = Position::all_board_positions().collect();

        // Should have 81 positions (9x9 grid)
        assert_eq!(positions.len(), 81);

        // Each position should be unique
        let unique_positions: HashSet<Point> = positions.iter().cloned().collect();
        assert_eq!(unique_positions.len(), 81);

        // Check that all combinations of x and y are present
        for x in Position::ALL_POSITIONS {
            for y in Position::ALL_POSITIONS {
                assert!(positions.contains(&Point::new(x, y)));
            }
        }
    }

    #[test]
    fn test_index_operations() {
        let mut cells = [Cell::Empty; 9];
        cells[Position::THREE] = Cell::from(3);
        cells[Position::SEVEN] = Cell::from(7);

        assert_eq!(cells[Position::THREE], Cell::from(3));
        assert_eq!(cells[Position::SEVEN], Cell::from(7));
        assert_eq!(cells[Position::ONE], Cell::Empty);

        // Test with 2D array
        let mut grid = [[Cell::Empty; 9]; 9];
        grid[Position::TWO][Position::FOUR] = Cell::from(5);

        assert_eq!(grid[Position::TWO][Position::FOUR], Cell::from(5));
        assert_eq!(grid[Position::TWO][Position::ONE], Cell::Empty);
    }

    #[test]
    fn test_random() {
        // Test that random() returns valid positions
        for _ in 0..100 {
            let pos = Position::random();
            assert!(pos.to_index() < 9);
        }

        // Test distribution of random values
        // (not a statistical test, just ensuring all values can appear)
        let mut seen = [false; 9];
        for _ in 0..1000 {
            let pos = Position::random();
            seen[pos.to_index()] = true;
        }

        // All positions should have been seen at least once
        assert!(seen.iter().all(|&x| x));
    }
}
