//! lib.rs
//! Implements the algorithm to be run as a PyPI python library
//! This Source Code Form is subject to the terms of The GNU General Public License v3.0
//! Copyright 2024 - Guilherme Santos. If a copy of the MPL was not distributed with this
//! file, You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html

mod mocd_nsga_ii;
mod mocd_pesa_ii;
mod pmoea;

mod graph;
mod operators;
mod utils;

pub use mocd_nsga_ii::MocdNsgaII;
pub use mocd_pesa_ii::MocdPesaII;
pub use pmoea::PMoEAE;

use pyo3::prelude::*;
use pyo3::types::PyDict;

// ================================================================================================
// Functions
// ================================================================================================

/// Calculates the Q score for a given graph and community partition
/// based on (Shi, 2012) multi-objective modularity equation. Q = 1 - intra - inter
///
/// # Parameters
/// - `graph` (networkx.Graph): The graph to analyze
/// - `partition` (dict[int, int]): Dictionary mapping nodes to community IDs
///
/// # Returns
/// - float
#[pyfunction(name = "fitness")]
fn fitness(graph: &Bound<'_, PyAny>, partition: &Bound<'_, PyDict>) -> PyResult<f64> {
    let edges = utils::get_edges(graph)?;
    let graph = utils::build_graph(edges);

    Ok(operators::get_modularity_from_partition(
        &utils::to_partition(partition)?,
        &graph,
    ))
}

// ================================================================================================
// Module
// ================================================================================================

#[pymodule]
#[pyo3(name = "pyevoea")]
fn pyevoea(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fitness, m)?)?;
    m.add_class::<MocdNsgaII>()?;
    m.add_class::<MocdPesaII>()?;
    m.add_class::<PMoEAE>()?;
    Ok(())
}
