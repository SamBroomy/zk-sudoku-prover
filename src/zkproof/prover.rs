use std::collections::HashMap;

use petgraph::graph::{EdgeIndex, EdgeIndices, NodeIndex};
use rand::seq::IteratorRandom;

use crate::{ColorShuffle, Commitment, CommitmentKey, Graph, Hidden, SudokuGrid};

use super::ZkProofError;

pub struct EdgeReveal {
    edge_index: EdgeIndex,
    node1: (NodeIndex, CommitmentKey),
    node2: (NodeIndex, CommitmentKey),
}

pub struct CommitmentSet {
    pub commitments: HashMap<NodeIndex, Commitment<Hidden>>, // Only contains the commitment hashes
    pub round_id: usize,
}

struct ProverRound {
    colour_shuffle: ColorShuffle,
    commitments: HashMap<NodeIndex, CommitmentKey>, // node_id -> commitment
    revealed_edges: Vec<EdgeIndex>,
}

pub struct Prover {
    graph: Graph,
    current_round: usize,
    rounds: Vec<ProverRound>,
}

impl Prover {
    pub fn new(puzzle: &SudokuGrid) -> Result<(Self, EdgeIndices), ZkProofError> {
        // Validate the Sudoku puzzle
        if !puzzle.is_valid_solution() {
            return Err(ZkProofError::SudokuError(
                "Invalid Sudoku puzzle".to_string(),
            ));
        }
        let graph = Graph::from_sudoku(puzzle);
        let edges = graph.edges();

        Ok((
            Self {
                graph,
                current_round: 0,
                rounds: Vec::new(),
            },
            edges,
        ))
    }

    pub fn start_round(&mut self) -> CommitmentSet {
        let colour_shuffle = ColorShuffle::new_random();

        let mut node_commitments = HashMap::with_capacity(self.graph.node_count());
        let mut node_keys = HashMap::with_capacity(self.graph.node_count());

        for (node_id, value) in self.graph.nodes() {
            let (commitment, commitment_key) =
                Commitment::new(colour_shuffle.apply(value), node_id.index());
            node_commitments.insert(node_id, commitment);
            node_keys.insert(node_id, commitment_key);
        }

        let prover_round = ProverRound {
            colour_shuffle,
            commitments: node_keys,
            revealed_edges: Vec::new(),
        };

        self.rounds.push(prover_round);
        self.current_round = self.rounds.len() - 1;

        CommitmentSet {
            commitments: node_commitments,
            round_id: self.current_round,
        }
    }

    pub fn reveal_edge(
        &mut self,
        round_id: usize,
        edge_idx: EdgeIndex,
    ) -> Result<EdgeReveal, ZkProofError> {
        if round_id != self.current_round {
            return Err(ZkProofError::RoundMismatch);
        }
        // TODO look at this.
        // if self.rounds[round_id].revealed_edges.contains(&edge_idx) {
        //     return Err(ZkProofError::AlreadyRevealed);
        // }

        let round = &mut self.rounds[round_id];

        let (node1, node2) = self
            .graph
            .get_edge_nodes(edge_idx)
            .map_err(|_| ZkProofError::EdgeNotFound(edge_idx))?;

        let node1_commitment_key = round
            .commitments
            .get(&node1)
            .ok_or(ZkProofError::NodeNotFound(node1.index()))?
            .to_owned();
        let node2_commitment_key = round
            .commitments
            .get(&node2)
            .ok_or(ZkProofError::NodeNotFound(node2.index()))?
            .to_owned();

        Ok(EdgeReveal {
            edge_index: edge_idx,
            node1: (node1, node1_commitment_key),
            node2: (node2, node2_commitment_key),
        })
    }
}

pub struct VerifierRound {
    commitments: CommitmentSet,
    selected_edge: Option<EdgeIndex>,
    revealed_edge: Option<EdgeReveal>,
    current_round: usize,
}

pub struct Verifier {
    indices: EdgeIndices,
    rounds: Vec<VerifierRound>,
}

impl Verifier {
    pub fn new(edges: EdgeIndices) -> Self {
        Self {
            indices: edges,
            rounds: Vec::new(),
        }
    }

    pub fn receive_commitments(
        &mut self,
        commitments: CommitmentSet,
    ) -> Result<EdgeIndex, ZkProofError> {
        if self.indices.len() == 0 {
            return Err(ZkProofError::NoEdges);
        }
        if self.rounds.len() != commitments.round_id {
            return Err(ZkProofError::RoundMismatch);
        }
        if self
            .rounds
            .iter()
            .any(|r| r.current_round == commitments.round_id)
        {
            return Err(ZkProofError::AlreadyRevealed);
        }

        let edge = self
            .indices
            .clone()
            .choose(&mut rand::rng())
            .ok_or(ZkProofError::NoEdges)?;

        let round = VerifierRound {
            commitments,
            selected_edge: Some(edge),
            revealed_edge: None,
            current_round: self.rounds.len(),
        };
        self.rounds.push(round);

        Ok(edge)
    }

    pub fn verify_edge_reveal(
        &mut self,
        EdgeReveal {
            edge_index,
            node1,
            node2,
        }: EdgeReveal,
    ) -> Result<(), ZkProofError> {
        let (node1_idx, node1_key) = node1;
        let (node2_idx, node2_key) = node2;
        let round = self
            .rounds
            .get_mut(edge_index.index())
            .ok_or(ZkProofError::RoundMismatch)?;

        if round.selected_edge != Some(edge_index) {
            return Err(ZkProofError::RoundMismatch);
        }

        // Verify the commitments
        let node1_commitment = round
            .commitments
            .commitments
            .get(&node1_idx)
            .ok_or(ZkProofError::NodeNotFound(node1_idx.index()))?;

        let node2_commitment = round
            .commitments
            .commitments
            .get(&node2_idx)
            .ok_or(ZkProofError::NodeNotFound(node2_idx.index()))?;

        // Verify the commitments
        let commit_one = node1_commitment.clone().reveal(node1_key)?;
        println!("Commitment 1 validated: {:?}", commit_one);

        let commit_two = node2_commitment.clone().reveal(node2_key)?;
        println!("Commitment 2 validated: {:?}", commit_two);

        Ok(())
    }

    pub fn confidence_level(&self) -> f64 {
        let edge_count = self.indices.len();
        if edge_count == 0 {
            return 0.0;
        }

        let successful_rounds = self
            .rounds
            .iter()
            .filter(|round| round.revealed_edge.is_some())
            .count();

        if successful_rounds == 0 {
            return 0.0;
        }
        // Probability of catching a cheating in any round
        let catch_prob = 1.0 / (edge_count as f64);

        // Probability of catching a cheater in at least one of N rounds
        // = 1 - (probability of not catching in any round)
        // = 1 - (1 - catch_prob)^N
        let confidence = 1.0 - (1.0 - catch_prob).powi(successful_rounds as i32);

        confidence * 100.0 // Return as percentage
    }
}

// Setup prover
//     Ingest the sudoku puzzle, create a graph, and emit the edges to the verifier
//
// Setup verifier
//    Receive the edges from the prover
//
// Start the round.
// -> Prover generates the commitments and shuffles the colors.
//     |--> This is in the form of a round and,
// -> Prover sends the commitments to the verifier.
//
// Verifier receives the commitments.
// The verifier will then select a random edge and send it back to the prover.
//
// Prover receives the edge and reveals the commitments for the two nodes connected by that edge.
//
// Verifier receives the revealed commitments and checks if they are valid.
