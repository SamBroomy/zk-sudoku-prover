// src/zkproof/types.rs
use crate::{
    CommitmentError,
    crypto::{Commitment, CommitmentKey, Hidden},
};
use petgraph::graph::{EdgeIndex, NodeIndex};
use std::collections::HashMap;
// Round identifier with newtype pattern for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoundId(pub usize);

pub type EdgeNodeMap = HashMap<EdgeIndex, (NodeIndex, NodeIndex)>;

#[derive(Debug, Clone)]
pub struct ProverCommitment {
    pub round_id: RoundId,
    pub commitments: HashMap<NodeIndex, Commitment<Hidden>>,
}

#[derive(Debug, Clone, Copy)]
pub struct VerifierChallenge {
    pub round_id: RoundId,
    pub edge: EdgeIndex,
}

pub struct NodeReveal {
    pub node_idx: NodeIndex,
    pub node_key: CommitmentKey,
}

pub struct ProverResponse {
    pub round_id: RoundId,
    pub edge: EdgeIndex,
    pub node1: NodeReveal,
    pub node2: NodeReveal,
}

#[derive(Debug, Clone, Copy)]
pub struct VerifierResult {
    pub round_id: RoundId,
    pub success: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ZkProofError {
    #[error("Node not found: {0}")]
    NodeNotFound(usize),
    #[error("Edge not found: {0:?}")]
    EdgeNotFound(EdgeIndex),
    #[error("Node mismatch: revealed nodes don't match the challenged edge")]
    NodeMismatch,
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
