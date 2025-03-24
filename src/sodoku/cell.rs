use super::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Cell {
    #[default]
    Empty,
    Guess(Value),
    Hint(Value),
}

impl Cell {
    pub fn new_empty() -> Self {
        Cell::Empty
    }
    pub fn new_guess(value: impl Into<Value>) -> Self {
        Cell::Guess(Value::new(value))
    }
    pub fn new_hint(value: impl Into<Value>) -> Self {
        Cell::Hint(Value::new(value))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    pub fn is_filled(&self) -> bool {
        !self.is_empty()
    }

    pub fn is_hint(&self) -> bool {
        matches!(self, Cell::Hint(_))
    }

    pub fn is_guess(&self) -> bool {
        matches!(self, Cell::Guess(_))
    }

    pub fn value(&self) -> Option<Value> {
        match self {
            Cell::Empty => None,
            Cell::Guess(val) => Some(*val),
            Cell::Hint(val) => Some(*val),
        }
    }
    // Add a method for the allocation-free Set validation
    pub fn value_as_index(&self) -> Option<usize> {
        self.value().map(|v| v.to_numeric() as usize)
    }

    pub fn hint_from_char(c: char) -> Self {
        match c {
            '1' => Cell::Hint(Value::One),
            '2' => Cell::Hint(Value::Two),
            '3' => Cell::Hint(Value::Three),
            '4' => Cell::Hint(Value::Four),
            '5' => Cell::Hint(Value::Five),
            '6' => Cell::Hint(Value::Six),
            '7' => Cell::Hint(Value::Seven),
            '8' => Cell::Hint(Value::Eight),
            '9' => Cell::Hint(Value::Nine),
            '.' | '0' | '_' => Cell::Empty,
            _ => panic!("Invalid character for cell: {}", c),
        }
    }
    pub fn guess_from_char(c: char) -> Self {
        match c {
            '1' => Cell::Guess(Value::One),
            '2' => Cell::Guess(Value::Two),
            '3' => Cell::Guess(Value::Three),
            '4' => Cell::Guess(Value::Four),
            '5' => Cell::Guess(Value::Five),
            '6' => Cell::Guess(Value::Six),
            '7' => Cell::Guess(Value::Seven),
            '8' => Cell::Guess(Value::Eight),
            '9' => Cell::Guess(Value::Nine),
            '.' | '0' | '_' => Cell::Empty,
            _ => panic!("Invalid character for cell: {}", c),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Guess(val) => write!(f, "{}", val),
            Cell::Hint(val) => write!(f, "{}", val),
        }
    }
}
