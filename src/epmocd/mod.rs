//! algorithms/pesa_ii.rs
//! Implements the Pareto Envelope-based Selection Algorithm II (PESA-II)
//! This Source Code Form is subject to the terms of The GNU General Public License v3.0
//! Copyright 2024 - Guilherme Santos. If a copy of the MPL was not distributed with this
//! file, You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html

mod evolutionary;
mod hypergrid;
mod refinement;

use refinement::{max_q_selection, refinement};
use rustc_hash::FxHashSet;
use crate::graph::{Graph, Partition, NodeId};
use hypergrid::{HyperBox, Solution};

use pyo3::{pyclass, pymethods};

use crate::utils::{build_graph, get_edges, normalize_community_ids};

use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyclass]
pub struct EpMoCD {
    graph: Graph,
    debug_level: i8,
    pop_size: usize,
    num_gens: usize,
    cross_rate: f64,
    mut_rate: f64,
}

impl EpMoCD {
    pub fn envolve(&self) -> Vec<Solution> {
        if self.debug_level >= 1 {
            self.graph.print();
        }

        evolutionary::evolutionary_phase(
            &self.graph,
            self.debug_level,
            self.num_gens,
            self.pop_size,
            self.cross_rate,
            self.mut_rate,
            &self.graph.precompute_degrees(),
        )
    }
}

#[pymethods]
impl EpMoCD {
    #[new]
    #[pyo3(signature = (graph,
        debug_level = 0,
        pop_size = 100,
        num_gens = 50,
        cross_rate = 0.8,
        mut_rate = 0.2
    ))]
    pub fn new(
        graph: &Bound<'_, PyAny>,
        debug_level: i8,
        pop_size: usize,
        num_gens: usize,
        cross_rate: f64,
        mut_rate: f64,
    ) -> PyResult<Self> {
        let edges = get_edges(graph)?;
        let graph = build_graph(edges);

        Ok(EpMoCD {
            graph,
            debug_level,
            pop_size,
            num_gens,
            cross_rate,
            mut_rate,
        })
    }

    #[pyo3(signature = ())]
    pub fn generate_pareto_front(&self) -> PyResult<Vec<(Partition, Vec<f64>)>> {
        let first_front = self.envolve();

        Ok(first_front
            .into_iter()
            .map(|ind| (normalize_community_ids(ind.partition), ind.objectives))
            .collect())
    }

    #[pyo3(signature = (resolution = 1.0, merge_threshold = None))]
    pub fn run(&self, resolution: f64, merge_threshold: Option<f64>) -> PyResult<Partition> {
        // Detect key nodes based on degree centrality
        let key_nodes: FxHashSet<NodeId> = self.graph.detect_key_nodes();
        
        // Generate Pareto front using the evolutionary phase
        let archive: Vec<Solution> = self.envolve();
        let solution = max_q_selection(&archive);
        
        // Apply enhanced refinement with resolution parameter and optional community merging
        let partition = refinement(
            solution.partition.clone(), 
            &archive, 
            &self.graph, 
            &key_nodes,
            resolution,
            merge_threshold,
        );

        // Normalize community IDs to ensure consecutive integers
        let normalized_partition = normalize_community_ids(partition);
        
        Ok(normalized_partition)
    }
}
