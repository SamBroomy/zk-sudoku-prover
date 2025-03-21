use petgraph::prelude::*;

use crate::Cell;

/// This graph is a colouring problem representation of a sudoku grid.
/// Each Node should be connected to all other nodes in the same row, column, box & also to
pub struct Graph {
    graph: UnGraph<Cell, ()>,
}
