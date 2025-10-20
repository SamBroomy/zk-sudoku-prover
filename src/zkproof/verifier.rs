use super::types::{
    EdgeNodeMap, ProverCommitment, ProverResponse, RoundId, VerifierChallenge, VerifierResult,
    ZkProofError,
};
use crate::NodeReveal;
use petgraph::graph::EdgeIndex;
use rand::{rng, seq::IteratorRandom};

pub struct VerifierRound {
    commitment: ProverCommitment,
    challenge_edge: EdgeIndex,
    response: Option<ProverResponse>,
    verified: bool,
}

pub struct Verifier {
    edge_map: EdgeNodeMap,
    rounds: Vec<VerifierRound>,
    current_round: RoundId,
}

impl Verifier {
    pub fn new(edge_map: EdgeNodeMap) -> Self {
        Self {
            edge_map,
            rounds: Vec::with_capacity(5_000), // Proof size for 99.4% confidence
            current_round: RoundId(0),
        }
    }

    pub fn receive_commitment(
        &mut self,
        commitment: ProverCommitment,
    ) -> Result<VerifierChallenge, ZkProofError> {
        // Validate round ID
        if commitment.round_id.0 != self.rounds.len() {
            return Err(ZkProofError::RoundMismatch);
        }
        if self.edge_map.is_empty() {
            return Err(ZkProofError::NoEdges);
        }

        let challenge_edge = *self
            .edge_map
            .keys()
            .choose(&mut rng())
            .ok_or(ZkProofError::NoEdges)?;

        let round_id = commitment.round_id;

        let round = VerifierRound {
            commitment,
            challenge_edge,
            response: None,
            verified: false,
        };

        self.rounds.push(round);
        self.current_round = round_id;

        Ok(VerifierChallenge {
            round_id,
            edge: challenge_edge,
        })
    }

    pub fn verify_response(
        &mut self,
        ProverResponse {
            round_id,
            edge,
            node1,
            node2,
        }: ProverResponse,
    ) -> Result<VerifierResult, ZkProofError> {
        if round_id != self.current_round {
            return Err(ZkProofError::RoundMismatch);
        }

        let round_idx = round_id.0;
        let round = self
            .rounds
            .get_mut(round_idx)
            .ok_or(ZkProofError::RoundMismatch)?;

        // Verify that its the edge we challenged
        if round.challenge_edge != edge {
            return Err(ZkProofError::RoundMismatch);
        }

        let (expected_node1, expected_node2) = self
            .edge_map
            .get(&edge)
            .ok_or(ZkProofError::EdgeNotFound(edge))?;

        let NodeReveal {
            node_idx: node1_idx,
            node_key: node1_key,
        } = node1;

        let NodeReveal {
            node_idx: node2_idx,
            node_key: node2_key,
        } = node2;

        // Verify that the nodes are the ones we expect
        if node1_idx != *expected_node1 || node2_idx != *expected_node2 {
            return Err(ZkProofError::NodeMismatch);
        }

        let node1_commitment = round
            .commitment
            .commitments
            .get(&node1_idx)
            .cloned()
            .ok_or(ZkProofError::NodeNotFound(node1_idx.index()))?;

        let node2_commitment = round
            .commitment
            .commitments
            .get(&node2_idx)
            .cloned()
            .ok_or(ZkProofError::NodeNotFound(node2_idx.index()))?;

        let node1_revealed = node1_commitment.reveal(node1_key)?;
        let node2_revealed = node2_commitment.reveal(node2_key)?;

        let success = node1_revealed.key().value() != node2_revealed.key().value();

        round.response = Some(ProverResponse {
            round_id,
            edge,
            node1: NodeReveal {
                node_idx: node1_idx,
                node_key: node1_revealed.key().clone(),
            },
            node2: NodeReveal {
                node_idx: node2_idx,
                node_key: node2_revealed.key().clone(),
            },
        });
        round.verified = success;

        Ok(VerifierResult { round_id, success })
    }

    pub fn edge_map_len(&self) -> usize {
        self.edge_map.len()
    }
    pub fn confidence_level(&self) -> f64 {
        let edge_count = self.edge_map.len();
        if edge_count == 0 {
            return 0.0;
        }

        let successful_rounds = self.rounds.iter().filter(|round| round.verified).count();

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

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, str::FromStr};

    use bytes::Bytes;
    use petgraph::graph::NodeIndex;

    use crate::{CommitmentKey, Prover, SudokuGrid, Value};

    use super::*;

    // Helper function to create a valid edge map for testing
    fn create_test_edge_map() -> HashMap<EdgeIndex, (NodeIndex, NodeIndex)> {
        let mut edge_map = HashMap::new();
        // Create 10 edges connecting 11 nodes
        for i in 0..10 {
            edge_map.insert(
                EdgeIndex::new(i),
                (NodeIndex::new(i), NodeIndex::new(i + 1)),
            );
        }
        edge_map
    }

    // Helper function to create a valid test commitment
    fn create_test_commitment(round_id: RoundId) -> ProverCommitment {
        // Create a valid Sudoku grid
        let grid_str =
            "296541378851273694743698251915764832387152946624839517139486725478325169562917483";
        let grid = SudokuGrid::from_str(grid_str).unwrap();

        // Create a prover with the grid
        let (mut prover, _) = Prover::new(&grid).unwrap();

        // Get a real commitment
        let commitment = prover.start_round();

        // Override the round_id
        ProverCommitment {
            round_id,
            commitments: commitment.commitments,
        }
    }

    #[test]
    fn test_verifier_creation() {
        let edge_map = create_test_edge_map();
        let verifier = Verifier::new(edge_map.clone());

        // Verify initial state
        assert_eq!(verifier.rounds.len(), 0);
        assert_eq!(verifier.current_round, RoundId(0));
        assert_eq!(verifier.edge_map.len(), edge_map.len());
    }

    #[test]
    fn test_receive_valid_commitment() {
        let edge_map = create_test_edge_map();
        let mut verifier = Verifier::new(edge_map);

        let commitment = create_test_commitment(RoundId(0));
        let challenge_result = verifier.receive_commitment(commitment);

        // Verify challenge was created
        assert!(challenge_result.is_ok());

        let challenge = challenge_result.unwrap();
        assert_eq!(challenge.round_id, RoundId(0));
        assert!(verifier.edge_map.contains_key(&challenge.edge));
    }

    #[test]
    fn test_receive_commitment_wrong_round_id() {
        let edge_map = create_test_edge_map();
        let mut verifier = Verifier::new(edge_map);

        // Create commitment with wrong round ID (should be 0)
        let commitment = create_test_commitment(RoundId(5));
        let result = verifier.receive_commitment(commitment);

        // Should fail with round mismatch
        assert!(matches!(result, Err(ZkProofError::RoundMismatch)));
    }

    #[test]
    fn test_receive_commitment_no_edges() {
        let empty_edge_map = HashMap::new();
        let mut verifier = Verifier::new(empty_edge_map);

        let commitment = create_test_commitment(RoundId(0));
        let result = verifier.receive_commitment(commitment);

        // Should fail with no edges
        assert!(matches!(result, Err(ZkProofError::NoEdges)));
    }

    #[test]
    fn test_multiple_rounds() {
        let edge_map = create_test_edge_map();
        let mut verifier = Verifier::new(edge_map);

        // Round 0
        let commitment0 = create_test_commitment(RoundId(0));
        let challenge0 = verifier.receive_commitment(commitment0).unwrap();
        assert_eq!(challenge0.round_id, RoundId(0));

        // Simulate successful verification for round 0
        verifier.rounds[0].verified = true;
        verifier.rounds[0].response = Some(ProverResponse {
            round_id: RoundId(0),
            edge: challenge0.edge,
            node1: NodeReveal {
                node_idx: NodeIndex::new(0),
                node_key: CommitmentKey::new_dummy(Value::One),
            },
            node2: NodeReveal {
                node_idx: NodeIndex::new(1),
                node_key: CommitmentKey::new_dummy(Value::Two),
            },
        });

        // Round 1
        let commitment1 = create_test_commitment(RoundId(1));
        let challenge1 = verifier.receive_commitment(commitment1).unwrap();
        assert_eq!(challenge1.round_id, RoundId(1));

        // Should have two rounds now
        assert_eq!(verifier.rounds.len(), 2);
        assert_eq!(verifier.current_round, RoundId(1));
    }

    #[test]
    fn test_verify_response_success() {
        // This requires integration with Prover
        // Setup grid and create prover + verifier
        let grid_str =
            "296541378851273694743698251915764832387152946624839517139486725478325169562917483";
        let grid = SudokuGrid::from_str(grid_str).unwrap();
        let (mut prover, edge_map) = Prover::new(&grid).unwrap();
        let mut verifier = Verifier::new(edge_map);

        // Start round and get commitment
        let commitment = prover.start_round();
        let challenge = verifier.receive_commitment(commitment).unwrap();

        // Prover generates response
        let response = prover.respond_to_challenge(challenge).unwrap();

        // Verifier verifies response
        let result = verifier.verify_response(response);

        // Should succeed
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_verify_response_wrong_round() {
        let edge_map = create_test_edge_map();
        let mut verifier = Verifier::new(edge_map);

        // Setup verifier with a round
        let commitment = create_test_commitment(RoundId(0));
        let challenge = verifier.receive_commitment(commitment).unwrap();

        // Create response with wrong round ID
        let response = ProverResponse {
            round_id: RoundId(5),
            edge: challenge.edge,
            node1: NodeReveal {
                node_idx: NodeIndex::new(0),
                node_key: CommitmentKey::new_dummy(Value::One),
            },
            node2: NodeReveal {
                node_idx: NodeIndex::new(1),
                node_key: CommitmentKey::new_dummy(Value::Two),
            },
        };

        // Should fail with round mismatch
        let result = verifier.verify_response(response);
        assert!(matches!(result, Err(ZkProofError::RoundMismatch)));
    }

    #[test]
    fn test_verify_response_wrong_edge() {
        let edge_map = create_test_edge_map();
        let mut verifier = Verifier::new(edge_map);

        // Setup verifier with a round
        let commitment = create_test_commitment(RoundId(0));
        verifier.receive_commitment(commitment).unwrap();

        // Create response with wrong edge
        let wrong_edge = EdgeIndex::new(99); // non-existent edge
        let response = ProverResponse {
            round_id: RoundId(0),
            edge: wrong_edge,
            node1: NodeReveal {
                node_idx: NodeIndex::new(0),
                node_key: CommitmentKey::new_dummy(Value::One),
            },
            node2: NodeReveal {
                node_idx: NodeIndex::new(1),
                node_key: CommitmentKey::new_dummy(Value::Two),
            },
        };

        // Should fail
        let result = verifier.verify_response(response);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_response_wrong_nodes() {
        // Similar to test_verify_response_success but with integration
        // We'd need a way to tamper with the nodes in the response
        // Testing by modifying the edge_map to expect different nodes

        // Setup valid edge map but modify one entry
        let mut edge_map = create_test_edge_map();
        let test_edge = EdgeIndex::new(0);
        edge_map.insert(test_edge, (NodeIndex::new(99), NodeIndex::new(100)));

        let mut verifier = Verifier::new(edge_map);

        // Setup verifier with a round and force the challenge edge
        let commitment = create_test_commitment(RoundId(0));
        verifier.receive_commitment(commitment).unwrap();
        verifier.rounds[0].challenge_edge = test_edge;

        // Create valid-looking but incorrect response
        let response = ProverResponse {
            round_id: RoundId(0),
            edge: test_edge,
            node1: NodeReveal {
                node_idx: NodeIndex::new(0), // Wrong node for the manipulated edge
                node_key: CommitmentKey::new_dummy(Value::One),
            },
            node2: NodeReveal {
                node_idx: NodeIndex::new(1), // Wrong node for the manipulated edge
                node_key: CommitmentKey::new_dummy(Value::Two),
            },
        };

        // Should fail with node mismatch
        let result = verifier.verify_response(response);
        assert!(matches!(result, Err(ZkProofError::NodeMismatch)));
    }

    #[test]
    fn test_confidence_level() {
        let edge_map = create_test_edge_map();
        let mut verifier = Verifier::new(edge_map);

        // With no rounds, confidence should be 0
        assert_eq!(verifier.confidence_level(), 0.0);

        // Add some verified rounds
        for i in 0..10 {
            let commitment = create_test_commitment(RoundId(i));
            verifier.receive_commitment(commitment).unwrap();
            verifier.rounds[i].verified = true;
        }

        // Now confidence should be higher
        let confidence = verifier.confidence_level();
        assert!(confidence > 0.0);
        assert!(confidence <= 100.0);

        // With more rounds, confidence should increase
        for i in 10..20 {
            let commitment = create_test_commitment(RoundId(i));
            verifier.receive_commitment(commitment).unwrap();
            verifier.rounds[i].verified = true;
        }

        let new_confidence = verifier.confidence_level();
        assert!(new_confidence > confidence);
    }

    #[test]
    fn test_full_zkproof_flow() {
        // Create valid grid
        let grid_str =
            "296541378851273694743698251915764832387152946624839517139486725478325169562917483";
        let grid = SudokuGrid::from_str(grid_str).unwrap();

        // Setup prover and verifier
        let (mut prover, edge_map) = Prover::new(&grid).unwrap();
        let mut verifier = Verifier::new(edge_map);

        // Run several rounds
        for _ in 0..100 {
            // Prover commits
            let commitment = prover.start_round();

            // Verifier issues challenge
            let challenge = verifier.receive_commitment(commitment).unwrap();

            // Prover responds
            let response = prover.respond_to_challenge(challenge).unwrap();

            // Verifier verifies
            let result = verifier.verify_response(response).unwrap();

            // Should be successful
            assert!(result.success);

            let confidence = verifier.confidence_level();
            println!("Confidence: {:.2}%", confidence);
        }

        // Check confidence level
        let confidence = verifier.confidence_level();
        println!("Confidence: {:.2}%", confidence);
        assert!(confidence > 9.0); // After 20 rounds, confidence should be around 9%
    }

    // We need to create a dummy CommitmentKey constructor for testing
    impl CommitmentKey {
        fn new_dummy(value: Value) -> Self {
            Self::new(value, Bytes::from_static(&[1, 2, 3, 4]))
        }
    }
}
