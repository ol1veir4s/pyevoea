//! algorithms/pesa_ii.rs
//! Implements the Pareto Envelope-based Selection Algorithm II (PESA-II)
//! This Source Code Form is subject to the terms of The GNU General Public License v3.0
//! Copyright 2024 - Guilherme Santos. If a copy of the MPL was not distributed with this
//! file, You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html

mod evolutionary;
mod hypergrid;
mod model_selection;

use crate::graph::{Graph, Partition};
use hypergrid::{HyperBox, Solution};

use pyo3::{pyclass, pymethods};

use crate::utils::{build_graph, get_edges, normalize_community_ids};

use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyclass]
pub struct MocdPesaII {
    graph: Graph,
    debug_level: i8,
    rand_networks: usize,
    pop_size: usize,
    num_gens: usize,
    cross_rate: f64,
    mut_rate: f64,
}

impl MocdPesaII {
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
impl MocdPesaII {
    #[new]
    #[pyo3(signature = (graph,
        debug_level = 0,
        rand_networks = 3,
        pop_size = 100,
        num_gens = 500,
        cross_rate = 0.8,
        mut_rate = 0.2
    ))]
    pub fn new(
        graph: &Bound<'_, PyAny>,
        debug_level: i8,
        rand_networks: usize,
        pop_size: usize,
        num_gens: usize,
        cross_rate: f64,
        mut_rate: f64,
    ) -> PyResult<Self> {
        let edges = get_edges(graph)?;
        let graph = build_graph(edges);

        Ok(MocdPesaII {
            graph,
            debug_level,
            rand_networks,
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

    #[pyo3(signature = ())]
    pub fn max_q(&self) -> PyResult<Partition> {
        let archive = self.envolve();
        let best_solution = model_selection::max_q_selection(&archive);

        Ok(normalize_community_ids(best_solution.partition.clone()))
    }

    pub fn min_max(&self) -> PyResult<Partition> {
        let archive = self.envolve();

        let best_solution = {
            let random_networks =
                model_selection::generate_random_networks(&self.graph, self.rand_networks);

            let random_archives: Vec<Vec<Solution>> = random_networks
                .iter()
                .map(|random_graph| {
                    let random_degrees = random_graph.precompute_degrees();
                    
                    evolutionary::evolutionary_phase(
                        random_graph,
                        self.debug_level,
                        self.num_gens,
                        self.pop_size,
                        self.cross_rate,
                        self.mut_rate,
                        &random_degrees,
                    )
                })
                .collect();
            model_selection::min_max_selection(&archive, &random_archives)
        };

        Ok(normalize_community_ids(best_solution.partition.clone()))
    }
}
