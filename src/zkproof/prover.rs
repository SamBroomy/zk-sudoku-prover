use itertools::Itertools;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use std::collections::HashMap;

use crate::{ColourShuffle, Commitment, CommitmentKey, Graph, SudokuGrid};

use super::{EdgeNodeMap, NodeReveal, ZkProofError};

use super::types::{ProverCommitment, ProverResponse, RoundId, VerifierChallenge};

pub struct ProverRound {
    colour_shuffle: ColourShuffle,
    commitment_keys: HashMap<NodeIndex, CommitmentKey>, // node_id -> commitment
    challenged_edges: Vec<EdgeIndex>,
}

pub struct Prover {
    graph: Graph,
    rounds: Vec<ProverRound>,
    current_round: RoundId,
}

impl Prover {
    pub fn new(puzzle: &SudokuGrid) -> Result<(Self, EdgeNodeMap), ZkProofError> {
        // Validate the Sudoku puzzle
        // if !puzzle.is_valid_solution() {
        //     return Err(ZkProofError::SudokuError(
        //         "Invalid Sudoku puzzle".to_string(),
        //     ));
        // }
        let graph = Graph::from_sudoku(puzzle);
        let mut edge_map = HashMap::with_capacity(graph.graph.edge_count());
        for edge_idx in graph.graph.edge_references() {
            edge_map.insert(edge_idx.id(), (edge_idx.source(), edge_idx.target()));
        }

        Ok((
            Self {
                graph,
                rounds: Vec::with_capacity(128),
                current_round: RoundId(0),
            },
            edge_map,
        ))
    }

    pub fn start_round(&mut self) -> ProverCommitment {
        let colour_shuffle = ColourShuffle::new_random();

        let (node_commitments, commitment_keys): (HashMap<_, _>, HashMap<_, _>) = self
            .graph
            .nodes()
            .map(|(node_id, value)| {
                let (commitment, key) =
                    Commitment::new(colour_shuffle.apply(value), node_id.index());
                ((node_id, commitment), (node_id, key))
            })
            .unzip();

        let round = ProverRound {
            colour_shuffle,
            commitment_keys,
            challenged_edges: Vec::new(),
        };

        let round_id = RoundId(self.rounds.len());
        self.rounds.push(round);
        self.current_round = round_id;
        ProverCommitment {
            round_id,
            commitments: node_commitments,
        }
    }

    pub fn respond_to_challenge(
        &mut self,
        challenge: VerifierChallenge,
    ) -> Result<ProverResponse, ZkProofError> {
        if challenge.round_id != self.current_round {
            return Err(ZkProofError::RoundMismatch);
        }

        let round_idx = challenge.round_id.0;
        let round = &mut self.rounds[round_idx];

        if round.challenged_edges.contains(&challenge.edge) {
            return Err(ZkProofError::AlreadyRevealed);
        }

        let (node1, node2) = self
            .graph
            .get_edge_nodes(challenge.edge)
            .map_err(|_| ZkProofError::EdgeNotFound(challenge.edge))?;

        let node1_key = round
            .commitment_keys
            .get(&node1)
            .ok_or(ZkProofError::NodeNotFound(node1.index()))?
            .clone();
        let node2_key = round
            .commitment_keys
            .get(&node2)
            .ok_or(ZkProofError::NodeNotFound(node2.index()))?
            .clone();

        round.challenged_edges.push(challenge.edge);

        Ok(ProverResponse {
            round_id: challenge.round_id,
            edge: challenge.edge,
            node1: NodeReveal {
                node_idx: node1,
                node_key: node1_key,
            },
            node2: NodeReveal {
                node_idx: node2,
                node_key: node2_key,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    fn create_valid_sudoku() -> SudokuGrid {
        SudokuGrid::from_str(
            "296541378851273694743698251915764832387152946624839517139486725478325169562917483",
        )
        .unwrap()
    }

    fn create_invalid_sudoku() -> SudokuGrid {
        SudokuGrid::from_str(
            "296541378851273694743698251915764832387152946624839517139486725478325169562917482",
        )
        .unwrap()
    }

    #[test]
    fn test_prover_creation_valid_sudoku() {
        let grid = create_valid_sudoku();
        let result = Prover::new(&grid);
        assert!(result.is_ok());

        let (_, edge_map) = result.unwrap();
        // Edge map should contain all edges in the graph
        assert!(!edge_map.is_empty());
    }

    #[test]
    fn test_prover_creation_invalid_sudoku() {
        let grid = create_invalid_sudoku();
        let result = Prover::new(&grid);
        assert!(result.is_err());

        assert!(matches!(result, Err(ZkProofError::SudokuError(_))));
    }

    #[test]
    fn test_start_round() {
        let grid = create_valid_sudoku();
        let (mut prover, _) = Prover::new(&grid).unwrap();

        // Start a round
        let commitment = prover.start_round();

        // Verify round ID is 0 for the first round
        assert_eq!(commitment.round_id, RoundId(0));

        // Check commitments exist for all nodes
        assert!(!commitment.commitments.is_empty());
    }

    #[test]
    fn test_multiple_rounds() {
        let grid = create_valid_sudoku();
        let (mut prover, _) = Prover::new(&grid).unwrap();

        // Start first round
        let commitment1 = prover.start_round();
        assert_eq!(commitment1.round_id, RoundId(0));

        // Start second round
        let commitment2 = prover.start_round();
        assert_eq!(commitment2.round_id, RoundId(1));

        // The commitments should be different due to different colour shuffles
        let mut all_different = true;
        for (node, comm1) in commitment1.commitments.iter() {
            if let Some(comm2) = commitment2.commitments.get(node) {
                if comm1.hash() == comm2.hash() {
                    all_different = false;
                    break;
                }
            }
        }
        assert!(
            all_different,
            "Commitments from different rounds should differ"
        );
    }

    #[test]
    fn test_respond_to_challenge() {
        let grid = create_valid_sudoku();
        let (mut prover, edge_map) = Prover::new(&grid).unwrap();

        // Start a round
        let commitment = prover.start_round();

        // Pick the first edge to challenge
        let edge = *edge_map.keys().next().unwrap();
        let challenge = VerifierChallenge {
            round_id: commitment.round_id,
            edge,
        };

        // Respond to challenge
        let response = prover.respond_to_challenge(challenge);
        assert!(response.is_ok());

        let response = response.unwrap();
        assert_eq!(response.round_id, commitment.round_id);
        assert_eq!(response.edge, edge);

        // Verify the nodes match the edge
        let (expected_node1, expected_node2) = edge_map[&edge];
        assert!(
            (response.node1.node_idx == expected_node1
                && response.node2.node_idx == expected_node2)
                || (response.node1.node_idx == expected_node2
                    && response.node2.node_idx == expected_node1)
        );
    }

    #[test]
    fn test_challenge_wrong_round() {
        let grid = create_valid_sudoku();
        let (mut prover, edge_map) = Prover::new(&grid).unwrap();

        // Start round 0
        prover.start_round();

        // Start round 1
        prover.start_round();

        // Create challenge for round 0 while round 1 is current
        let edge = *edge_map.keys().next().unwrap();
        let challenge = VerifierChallenge {
            round_id: RoundId(0),
            edge,
        };

        // Should fail due to round mismatch
        let response = prover.respond_to_challenge(challenge);
        assert!(matches!(response, Err(ZkProofError::RoundMismatch)));
    }

    #[test]
    fn test_challenge_same_edge_twice() {
        let grid = create_valid_sudoku();
        let (mut prover, edge_map) = Prover::new(&grid).unwrap();

        // Start a round
        let commitment = prover.start_round();

        // Pick an edge to challenge
        let edge = *edge_map.keys().next().unwrap();
        let challenge = VerifierChallenge {
            round_id: commitment.round_id,
            edge,
        };

        // First challenge should succeed
        assert!(prover.respond_to_challenge(challenge).is_ok());

        // Second challenge for the same edge should fail
        let result = prover.respond_to_challenge(challenge);
        assert!(matches!(result, Err(ZkProofError::AlreadyRevealed)));
    }

    #[test]
    fn test_challenge_nonexistent_edge() {
        let grid = create_valid_sudoku();
        let (mut prover, edge_map) = Prover::new(&grid).unwrap();

        // Start a round
        let commitment = prover.start_round();

        // Create a challenge with an invalid edge index
        let max_edge_index = edge_map.keys().map(|e| e.index()).max().unwrap_or(0);
        let invalid_edge = petgraph::graph::EdgeIndex::new(max_edge_index + 1);

        let challenge = VerifierChallenge {
            round_id: commitment.round_id,
            edge: invalid_edge,
        };

        // Should fail with EdgeNotFound
        let result = prover.respond_to_challenge(challenge);
        assert!(matches!(result, Err(ZkProofError::EdgeNotFound(_))));
    }

    #[test]
    fn test_revealed_values_valid_for_edge() {
        let grid = create_valid_sudoku();
        let (mut prover, edge_map) = Prover::new(&grid).unwrap();

        // Start a round
        let commitment = prover.start_round();

        // Challenge all edges
        for edge in edge_map.keys() {
            // Create a challenge
            let challenge = VerifierChallenge {
                round_id: commitment.round_id,
                edge: *edge,
            };

            // Get the response
            let response = prover.respond_to_challenge(challenge).unwrap();

            // Verify the revealed values are different (adjacent nodes must have different colours)
            assert_ne!(
                response.node1.node_key.value(),
                response.node2.node_key.value(),
                "Connected nodes should have different values"
            );
        }
    }
}
