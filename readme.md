# ZK-Sudoku-Prover

A Rust implementation of a zero-knowledge proof system for Sudoku puzzles, based on graph colouring.

## Overview

This project demonstrates how a prover can convince a verifier that they've solved a Sudoku puzzle without revealing the actual solution. It implements the concepts discussed in [this video](https://www.youtube.com/watch?v=Otvcbw6k4eo). I liked the video and wanted to try and implementing that style of zero-knowledge proof in Rust.

The implementation uses:

- Reduction of Sudoku to a graph colouring problem
- Commitment schemes with cryptographic hashing
- Statistical verification through multiple interactive rounds
- Color shuffling to maintain the zero-knowledge property

## Project Structure

```project_structure
src/
├── crypto/              - Cryptographic primitives
│   ├── colour_shuffle.rs - Shuffling of colors between rounds
│   ├── commitment.rs    - Commitment scheme implementation
│   └── mod.rs
├── graph/               - Graph representation
│   └── mod.rs           - Conversion from Sudoku to graph colouring
├── sodoku/              - Sudoku representation
│   ├── cell.rs          - Cell representation
│   ├── grid.rs          - Full grid with validation
│   ├── mod.rs
│   ├── point.rs         - Grid coordinate handling
│   ├── position.rs      - Position abstractions
│   ├── set.rs           - Row/Column/Box abstractions
│   └── value.rs         - Sudoku values (1-9)
├── zkproof/             - Zero-knowledge proof protocol
│   ├── mod.rs
│   ├── protocol.rs      - Main protocol orchestration
│   ├── prover.rs        - Prover implementation
│   ├── types.rs         - Protocol data types
│   └── verifier.rs      - Verifier implementation
├── lib.rs               - Library exports
└── main.rs              - Server entry point
```

## Rust Implementation Highlights

### Type Safety and Ownership

This implementation leverages Rust's strong type system to enforce cryptographic security guarantees at compile time:

1. **Typed Commitments**: The commitment system uses generic state parameters to enforce proper usage:

   ```rust
   pub struct Commitment<S = Hidden> {
       hash: Bytes,
       node_id: usize,
       key: Option<CommitmentKey>,
       _marker: PhantomData<S>,
   }
   ```

   - You cannot access a commitment's value without first revealing it with the correct key
   - The type system prevents accidental or malicious revelation
   - The `reveal()` method transforms a `Commitment<Hidden>` into a `Commitment<Revealed>` only when the correct key is provided

2. **Strong Enums for Domain Values**: Using enums instead of primitive types for Sudoku values and positions:

   ```rust
   pub enum Value {
       One, Two, Three, Four, Five, Six, Seven, Eight, Nine
   }
   ```

   - Makes impossible states unrepresentable
   - Eliminates whole classes of bugs like out-of-range values

3. **Ownership-Based Protocol Flow**: The protocol enforces correct sequencing through ownership transfers:
   - Commitments must be created before they can be revealed
   - Once revealed, the hidden commitment is consumed
   - The verifier cannot check a commitment without it being properly revealed first

4. **Zero-Cost Abstractions**: The type safety has no runtime cost due to Rust's zero-cost abstractions

## Core Concepts

### 1. Reduction to Graph Coloring

The Sudoku grid is transformed into a graph where:

- Each cell is a node
- Two nodes are connected by an edge if their values must be different
- Cells in the same row, column, or 3x3 box are connected
- Pre-filled hints are enforced through special clique connections

### 2. Zero-Knowledge Protocol

The interactive protocol works as follows:

1. **Commitment Phase**:
   - Prover randomly shuffles the colors (values 1-9)
   - For each node, prover creates a cryptographic commitment to its color
   - Commitments are sent to the verifier

2. **Challenge Phase**:
   - Verifier randomly selects an edge to check
   - Prover reveals only the colors of the two nodes connected by that edge

3. **Verification Phase**:
   - Verifier confirms that the revealed values are different
   - Verifier confirms that the revealed commitments match the original ones

4. **Repeat**:
   - Multiple rounds are executed with fresh color shuffling each time
   - Statistical confidence increases with each round

### 3. Commitment Scheme

The commitment scheme uses Blake3 cryptographic hashing with:

- A value (1-9) to commit to
- A random nonce to prevent guessing
- Type-safe states (Hidden/Revealed) to prevent premature revelation

## Usage

### Example

```rust
use std::str::FromStr;
use zk_sudoku_prover::*;

fn main() {
    // Parse a solved Sudoku grid
    let grid_str = "296541378851273694743698251915764832387152946624839517139486725478325169562917483";
    let board = SudokuGrid::from_str(grid_str).unwrap();

    // Create the zero-knowledge protocol
    let mut zk_protocol = ZKProtocol::new(&board).unwrap();

    // Run the proof with 99% confidence
    let result = zk_protocol.prove_with_confidence(99.0).unwrap();

    println!("Proof verification: {}", result);
}
```

### Running Example

```bash
cargo run --release
```

## How It Works

### Commitment Creation

```rust
// Create a commitment to a specific value
let (commitment, key) = Commitment::new(Value::Five, node_id);

// Later, reveal the commitment
let revealed = commitment.reveal(key)?;
```

### Color Shuffling

```rust
// Create a random permutation of colors
let shuffle = ColourShuffle::new_random();

// Map original value to shuffled value
let shuffled_value = shuffle.apply(original_value);

// Map back when needed
let original_value = shuffle.reverse_apply(shuffled_value);
```

### Confidence Calculation

The protocol calculates how many rounds are needed to achieve a desired confidence level:

```rust
pub fn calculate_rounds_needed(edge_count: usize, confidence: f64) -> usize {
    let catch_prob = 1.0 / (edge_count as f64);
    let log_term = (1.0 - confidence / 100.0).ln() / (1.0 - catch_prob).ln();
    log_term.ceil() as usize
}
```

## Typesafe Commitment System

The commitment system is implemented with type-level guarantees:

```rust
// Creating a commitment returns a hidden commitment and a separate key
let (commitment, key) = Commitment::new(Value::Five, node_id);
// Type: Commitment<Hidden>, CommitmentKey

// Without the correct key, you can't access the committed value
// commitment.value()  // Would not compile!

// Revealing requires the correct key and consumes the hidden commitment
let revealed_commitment = commitment.reveal(key)?;
// Type: Commitment<Revealed>

// Now we can safely access the value
let value = revealed_commitment.key().value();
```

This type-driven approach enforces that:

1. Hidden commitments cannot reveal their values
2. Only the holder of the correct key can reveal a commitment
3. The state transition from Hidden to Revealed is explicit and verified

The implementation uses Rust's type system to make invalid operations impossible at compile time, rather than having to check for them at runtime.

## Cryptographic Security

The implementation ensures:

1. **Commitment Binding**: Prover cannot change committed values
2. **Commitment Hiding**: Verifier learns nothing from commitments alone
3. **Zero-Knowledge**: Verifier learns only that connected nodes have different colors
4. **Statistical Soundness**: With enough rounds, chance of successful cheating approaches zero

## Performance

Current implementation performance:

- Typical proof time: ~96ms for a standard Sudoku grid
- Confidence: Can achieve 99% confidence in approximately 4500 rounds
