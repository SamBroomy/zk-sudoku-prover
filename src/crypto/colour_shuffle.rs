use crate::Value;
use rand::rng;
use rand::seq::SliceRandom;

/// A permutation of colours (values 1-9)
#[derive(Debug, Clone)]
pub struct ColourShuffle {
    /// Maps from original value to shuffled value (0-indexed)
    value_map: [Value; 9],
}

impl ColourShuffle {
    /// Create a new random colour shuffle
    pub fn new_random() -> Self {
        let mut rng = rng();
        let mut values = Value::ALL_VALUES;
        values.shuffle(&mut rng);

        Self { value_map: values }
    }

    pub fn apply(&self, value: Value) -> Value {
        self.value_map[value.to_index()]
    }

    /// Apply the inverse of the shuffle
    pub fn reverse_apply(&self, value: Value) -> Value {
        let index = self.value_map.iter().position(|&v| v == value).unwrap();
        Value::from_index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colour_shuffle() {
        let shuffle = ColourShuffle::new_random();
        let original = Value::One;
        let shuffled = shuffle.apply(original);
        let reversed = shuffle.reverse_apply(shuffled);

        assert_eq!(original, reversed);
    }
}
