use crate::Value;
use rand::rng;
use rand::seq::SliceRandom;

/// A permutation of colors (values 1-9)
#[derive(Debug, Clone)]
pub struct ColorShuffle {
    /// Maps from original value to shuffled value (0-indexed)
    value_map: [Value; 9],
}

impl ColorShuffle {
    /// Create a new random color shuffle
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
    fn test_color_shuffle() {
        let shuffle = ColorShuffle::new_random();
        let original = Value::One;
        let shuffled = shuffle.apply(original);
        let reversed = shuffle.reverse_apply(shuffled);

        assert_eq!(original, reversed);
    }
}
