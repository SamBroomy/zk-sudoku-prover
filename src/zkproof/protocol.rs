use crate::SudokuGrid;

use super::{Prover, Verifier, VerifierResult, ZkProofError};

pub struct ZKProtocol {
    prover: Prover,
    verifier: Verifier,
}

impl ZKProtocol {
    pub fn new(puzzle: &SudokuGrid) -> Result<Self, ZkProofError> {
        let (prover, edge_map) = Prover::new(puzzle)?;
        let verifier = Verifier::new(edge_map);
        Ok(Self { prover, verifier })
    }

    pub fn run_round(&mut self) -> Result<VerifierResult, ZkProofError> {
        // Step 1: Prover generates commitments
        let commitments = self.prover.start_round();

        // Step 2: Verifier receives commitments & selects a random edge
        let challenge_edge = self.verifier.receive_commitment(commitments)?;

        // Step 3: Prover reveals the nodes of the edge
        let response = self.prover.respond_to_challenge(challenge_edge)?;

        // Step 4: Verifier verifies the response
        self.verifier.verify_response(response)
    }

    /// Run multiple rounds of the protocol
    pub fn run_proof(&mut self, num_rounds: usize) -> Result<bool, ZkProofError> {
        for round in 1..=num_rounds {
            if !self.run_round()?.success {
                println!("Failed verification in round {}", round);
                return Ok(false); // Failed verification
            }
        }

        Ok(true) // All rounds successful
    }

    pub fn prove_with_confidence(&mut self, confidence: f64) -> Result<bool, ZkProofError> {
        println!("Desired confidence: {}", confidence);
        let edge_count = self.verifier.edge_map_len();
        let rounds_needed = Self::calculate_rounds_needed(edge_count, confidence);
        println!(
            "Running {} rounds for {:.2}% confidence",
            rounds_needed, confidence
        );
        self.run_proof(rounds_needed)
    }

    pub fn calculate_rounds_needed(edge_count: usize, confidence: f64) -> usize {
        let catch_prob = 1.0 / (edge_count as f64);
        let log_term = (1.0 - confidence / 100.0).ln() / (1.0 - catch_prob).ln();
        log_term.ceil() as usize
    }
}

// Setup prover
//     Ingest the sudoku puzzle, create a graph, and emit the edges to the verifier
//
// Setup verifier
//    Receive the edges from the prover
//
// Start the round.
// -> Prover generates the commitments and shuffles the colours.
//     |--> This is in the form of a round and,
// -> Prover sends the commitments to the verifier.
//
// Verifier receives the commitments.
// The verifier will then select a random edge and send it back to the prover.
//
// Prover receives the edge and reveals the commitments for the two nodes connected by that edge.
//
// Verifier receives the revealed commitments and checks if they are valid.
