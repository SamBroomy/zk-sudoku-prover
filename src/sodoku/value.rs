use core::fmt;

use num_traits::NumCast;

/// Represents the values in a Sudoku grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Value {
    pub const ALL_VALUES: [Value; 9] = [
        Value::One,
        Value::Two,
        Value::Three,
        Value::Four,
        Value::Five,
        Value::Six,
        Value::Seven,
        Value::Eight,
        Value::Nine,
    ];

    // A constructor that panics on invalid input
    pub fn new<T: Into<Self>>(value: T) -> Self {
        value.into()
    }

    pub fn to_numeric(self) -> u8 {
        match self {
            Value::One => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
        }
    }

    pub fn to_index(self) -> usize {
        match self {
            Value::One => 0,
            Value::Two => 1,
            Value::Three => 2,
            Value::Four => 3,
            Value::Five => 4,
            Value::Six => 5,
            Value::Seven => 6,
            Value::Eight => 7,
            Value::Nine => 8,
        }
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Value::One,
            1 => Value::Two,
            2 => Value::Three,
            3 => Value::Four,
            4 => Value::Five,
            5 => Value::Six,
            6 => Value::Seven,
            7 => Value::Eight,
            8 => Value::Nine,
            _ => panic!("Invalid index for value: {}", index),
        }
    }
}

impl Value {
    /// Creates a Value from any numeric type
    pub fn from_number<T>(num: T) -> Self
    where
        T: NumCast + Copy + fmt::Display,
    {
        // Convert to u8 safely
        let n: Option<u8> = NumCast::from(num);
        match n {
            Some(1) => Value::One,
            Some(2) => Value::Two,
            Some(3) => Value::Three,
            Some(4) => Value::Four,
            Some(5) => Value::Five,
            Some(6) => Value::Six,
            Some(7) => Value::Seven,
            Some(8) => Value::Eight,
            Some(9) => Value::Nine,
            Some(n) => panic!("Invalid value: {}, must be between 1 and 9", n),
            None => panic!("Invalid value: {}, cannot be converted to u8", num),
        }
    }
}

// Implement TryFrom for common integer types
macro_rules! impl_from_for_value {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Value {
                fn from(value: $t) -> Self {
                    Value::from_number(value)
                }
            }
        )*
    };
}
impl_from_for_value!(u8, u16, u32, i8, i16, i32, i64, u64, usize, isize);

impl From<char> for Value {
    fn from(c: char) -> Self {
        match c {
            '1' => Value::One,
            '2' => Value::Two,
            '3' => Value::Three,
            '4' => Value::Four,
            '5' => Value::Five,
            '6' => Value::Six,
            '7' => Value::Seven,
            '8' => Value::Eight,
            '9' => Value::Nine,
            _ => panic!("Invalid character for value: {}", c),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::One => write!(f, "1"),
            Value::Two => write!(f, "2"),
            Value::Three => write!(f, "3"),
            Value::Four => write!(f, "4"),
            Value::Five => write!(f, "5"),
            Value::Six => write!(f, "6"),
            Value::Seven => write!(f, "7"),
            Value::Eight => write!(f, "8"),
            Value::Nine => write!(f, "9"),
        }
    }
}
