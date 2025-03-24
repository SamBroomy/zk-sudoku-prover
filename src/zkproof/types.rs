// src/zkproof/types.rs
use petgraph::graph::EdgeIndex;

use crate::CommitmentError;

#[derive(Debug, Clone)]
pub struct EdgeReveal {
    pub edge_index: EdgeIndex,
    pub node1_id: usize,
    pub node1_value: u8,
    pub node1_nonce: Vec<u8>,
    pub node2_id: usize,
    pub node2_value: u8,
    pub node2_nonce: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum ZkProofError {
    #[error("Node not found: {0}")]
    NodeNotFound(usize),
    #[error("Edge not found: {0:?}")]
    EdgeNotFound(EdgeIndex),
    #[error("Invalid reveal: hash doesn't match")]
    InvalidReveal(#[from] CommitmentError),
    #[error("No edges available")]
    NoEdges,
    #[error("Round mismatch")]
    RoundMismatch,
    #[error("Commitment already revealed")]
    AlreadyRevealed,
    #[error("Value not found for node")]
    ValueNotFound,
    #[error("Graph error: {0}")]
    GraphError(String),
    #[error("Sudoku error: {0}")]
    SudokuError(String),
}
