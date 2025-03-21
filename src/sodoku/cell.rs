use std::str::FromStr;

use rand::seq::IndexedRandom;

use super::Position;

/// Represents a the possible values for a cell in a Sudoku puzzle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Cell {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    #[default]
    Empty,
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }
    pub fn is_filled(&self) -> bool {
        !self.is_empty()
    }

    pub fn random() -> Self {
        let variants = [
            Cell::One,
            Cell::Two,
            Cell::Three,
            Cell::Four,
            Cell::Five,
            Cell::Six,
            Cell::Seven,
            Cell::Eight,
            Cell::Nine,
            Cell::Empty,
        ];
        *variants.choose(&mut rand::rng()).unwrap()
    }

    // Version that excludes Empty
    pub fn random_value() -> Self {
        let variants = [
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
        *variants.choose(&mut rand::rng()).unwrap()
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            1 => Cell::One,
            2 => Cell::Two,
            3 => Cell::Three,
            4 => Cell::Four,
            5 => Cell::Five,
            6 => Cell::Six,
            7 => Cell::Seven,
            8 => Cell::Eight,
            9 => Cell::Nine,
            0 => Cell::Empty,
            _ => panic!("Invalid value for cell: {}", value),
        }
    }
}
impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '1' => Cell::One,
            '2' => Cell::Two,
            '3' => Cell::Three,
            '4' => Cell::Four,
            '5' => Cell::Five,
            '6' => Cell::Six,
            '7' => Cell::Seven,
            '8' => Cell::Eight,
            '9' => Cell::Nine,
            '.' | '_' | '0' => Cell::Empty,
            _ => panic!("Invalid character for cell: {}", value),
        }
    }
}

impl FromStr for Cell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u8>() {
            Ok(value) => Ok(Cell::from(value)),
            Err(_) => Err(()),
        }
    }
}
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::One => write!(f, "1"),
            Cell::Two => write!(f, "2"),
            Cell::Three => write!(f, "3"),
            Cell::Four => write!(f, "4"),
            Cell::Five => write!(f, "5"),
            Cell::Six => write!(f, "6"),
            Cell::Seven => write!(f, "7"),
            Cell::Eight => write!(f, "8"),
            Cell::Nine => write!(f, "9"),
            Cell::Empty => write!(f, "."),
        }
    }
}

impl From<Position> for Cell {
    fn from(value: Position) -> Self {
        match value {
            Position::ONE => Cell::One,
            Position::TWO => Cell::Two,
            Position::THREE => Cell::Three,
            Position::FOUR => Cell::Four,
            Position::FIVE => Cell::Five,
            Position::SIX => Cell::Six,
            Position::SEVEN => Cell::Seven,
            Position::EIGHT => Cell::Eight,
            Position::NINE => Cell::Nine,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_is_empty() {
        assert!(Cell::Empty.is_empty());
        assert!(!Cell::One.is_empty());
        assert!(!Cell::Nine.is_empty());
    }

    #[test]
    fn test_is_filled() {
        assert!(!Cell::Empty.is_filled());
        assert!(Cell::One.is_filled());
        assert!(Cell::Nine.is_filled());
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(Cell::from(1), Cell::One);
        assert_eq!(Cell::from(5), Cell::Five);
        assert_eq!(Cell::from(9), Cell::Nine);
        assert_eq!(Cell::from(0), Cell::Empty);
    }

    #[test]
    #[should_panic(expected = "Invalid value for cell: 10")]
    fn test_from_u8_invalid() {
        let _ = Cell::from(10);
    }

    #[test]
    fn test_from_char() {
        assert_eq!(Cell::from('1'), Cell::One);
        assert_eq!(Cell::from('5'), Cell::Five);
        assert_eq!(Cell::from('9'), Cell::Nine);
        assert_eq!(Cell::from('.'), Cell::Empty);
        assert_eq!(Cell::from('0'), Cell::Empty);
        assert_eq!(Cell::from('_'), Cell::Empty);
    }

    #[test]
    #[should_panic(expected = "Invalid character for cell: a")]
    fn test_from_char_invalid() {
        let _ = Cell::from('a');
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Cell::from_str("1").unwrap(), Cell::One);
        assert_eq!(Cell::from_str("5").unwrap(), Cell::Five);
        assert_eq!(Cell::from_str("9").unwrap(), Cell::Nine);
        assert_eq!(Cell::from_str("0").unwrap(), Cell::Empty);
        assert!(Cell::from_str("invalid").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(Cell::One.to_string(), "1");
        assert_eq!(Cell::Five.to_string(), "5");
        assert_eq!(Cell::Nine.to_string(), "9");
        assert_eq!(Cell::Empty.to_string(), ".");
    }

    #[test]
    fn test_from_position() {
        assert_eq!(Cell::from(Position::ONE), Cell::One);
        assert_eq!(Cell::from(Position::FIVE), Cell::Five);
        assert_eq!(Cell::from(Position::NINE), Cell::Nine);
    }

    #[test]
    fn test_random_value_is_not_empty() {
        for _ in 0..100 {
            let cell = Cell::random_value();
            assert!(cell.is_filled());
        }
    }

    #[test]
    fn test_random_can_generate_all_values() {
        let mut seen = [false; 10]; // 0-9 (including Empty)

        // Run many times to have a good chance of seeing all values
        for _ in 0..1000 {
            let cell = Cell::random();

            // Convert to index
            let idx = match cell {
                Cell::One => 1,
                Cell::Two => 2,
                Cell::Three => 3,
                Cell::Four => 4,
                Cell::Five => 5,
                Cell::Six => 6,
                Cell::Seven => 7,
                Cell::Eight => 8,
                Cell::Nine => 9,
                Cell::Empty => 0,
            };

            seen[idx] = true;
        }

        // Verify all variants were seen
        for (i, &was_seen) in seen.iter().enumerate() {
            assert!(was_seen, "Value {} was never generated", i);
        }
    }
}
