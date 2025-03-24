use crate::SudokuGrid;

use super::{Prover, Verifier, ZkProofError};

pub struct ZKProtocol {
    prover: Prover,
    verifier: Verifier,
}

impl ZKProtocol {
    pub fn new(puzzle: &SudokuGrid) -> Result<Self, ZkProofError> {
        let (prover, edges) = Prover::new(puzzle)?;
        let verifier = Verifier::new(edges);
        Ok(Self { prover, verifier })
    }

    pub fn run_round(&mut self) -> Result<(), ZkProofError> {
        // Step 1: Prover generates commitments
        let commitments = self.prover.start_round();
        let round_id = commitments.round_id;

        // Step 2: Verifier receives commitments & selects a random edge
        let challenge_edge = self.verifier.receive_commitments(commitments)?;

        // Step 3: Prover reveals the nodes of the edge
        let revealed_edges = self.prover.reveal_edge(round_id, challenge_edge)?;

        // Step 4: Verifier verifies the revealed nodes
        self.verifier.verify_edge_reveal(revealed_edges)?;

        Ok(())
    }
}
