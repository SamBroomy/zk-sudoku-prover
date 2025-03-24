use petgraph::graph::{EdgeIndex, EdgeIndices, NodeIndex, UnGraph};

use crate::{Cell, Point, Position, SudokuGrid, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SudokuNode {
    cell: Value,
    location: Point,
}

/// This graph is a colouring problem representation of a sudoku grid.
/// Each Node should be connected to all other nodes in the same row, column, box & also to clues.
/// The clues are not connected to each other but to a clique set of nine special nodes each corresponding to a number.
/// The clue node are forced to have a particular value by connecting it to all the clique nodes except the one corresponding to its value.
pub struct Graph {
    pub graph: UnGraph<SudokuNode, ()>,
}

impl Graph {
    pub fn from_sudoku(sudoku: &SudokuGrid) -> Self {
        let mut graph = UnGraph::new_undirected();

        // Create nodes for each cell in the grid
        let mut cell_nodes = [[NodeIndex::new(0); 9]; 9];
        for x in Position::ALL_POSITIONS {
            for y in Position::ALL_POSITIONS {
                let point = Point::new(x, y);
                let cell = sudoku.get_cell(point);

                // Use cell's value if it has one, otherwise default to One
                let node_value = cell.value().unwrap();

                let node_index = graph.add_node(SudokuNode {
                    cell: node_value,
                    location: point,
                });

                cell_nodes[x.to_index()][y.to_index()] = node_index;
            }
        }

        // Create the 9 special clique nodes (one for each value 1-9)
        let mut clique_nodes = Vec::with_capacity(9);
        for i in Value::ALL_VALUES {
            let node_index = graph.add_node(SudokuNode {
                cell: i,
                location: Point::default(), // Clique nodes don't have a grid location
            });
            clique_nodes.push(node_index);
        }

        // Connect cells in the same row
        for row in &cell_nodes {
            for i in 0..8 {
                for j in (i + 1)..9 {
                    graph.add_edge(row[i], row[j], ());
                }
            }
        }

        // Connect cells in the same column
        for col_idx in 0..9 {
            for i in 0..8 {
                for j in (i + 1)..9 {
                    graph.add_edge(cell_nodes[i][col_idx], cell_nodes[j][col_idx], ());
                }
            }
        }

        // Connect cells in the same box/square
        for box_row in 0..3 {
            for box_col in 0..3 {
                let start_row = box_row * 3;
                let start_col = box_col * 3;

                // Collect all nodes in this box
                let mut box_nodes = Vec::with_capacity(9);
                for r in 0..3 {
                    for c in 0..3 {
                        let row_idx = start_row + r;
                        let col_idx = start_col + c;
                        box_nodes.push(cell_nodes[row_idx][col_idx]);
                    }
                }

                // Connect each cell to all others in the box
                for i in 0..8 {
                    for j in (i + 1)..9 {
                        graph.add_edge(box_nodes[i], box_nodes[j], ());
                    }
                }
            }
        }

        // Connect hint cells to clique nodes
        for x in Position::ALL_POSITIONS {
            for y in Position::ALL_POSITIONS {
                let point = Point::new(x, y);
                let cell = sudoku.get_cell(point);

                if let Cell::Hint(value) = cell {
                    let cell_node = cell_nodes[x.to_index()][y.to_index()];
                    let value_idx = value.to_numeric() as usize - 1; // Convert 1-9 to 0-8

                    // Connect to all clique nodes EXCEPT the one matching its value
                    for (i, &clique_node) in clique_nodes.iter().enumerate() {
                        if i != value_idx {
                            graph.add_edge(cell_node, clique_node, ());
                        }
                    }
                }
            }
        }

        Self { graph }
    }
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }
    /// Get all nodes in the graph
    pub fn nodes(&self) -> impl Iterator<Item = (NodeIndex, Value)> {
        self.graph
            .node_indices()
            .map(|idx| (idx, self.graph[idx].cell))
    }

    /// Get all edges in the graph
    pub fn edges(&self) -> EdgeIndices {
        self.graph.edge_indices()
    }

    /// Get the nodes connected by an edge
    pub fn get_edge_nodes(&self, edge: EdgeIndex) -> Result<(NodeIndex, NodeIndex), GraphError> {
        let (a, b) = self
            .graph
            .edge_endpoints(edge)
            .ok_or(GraphError::EdgeNotFound)?;
        Ok((a, b))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("Edge not found")]
    EdgeNotFound,
}
