use crate::{epmocd::Solution, graph::{Graph, Partition, NodeId, CommunityId}};
use rustc_hash::{FxHashSet, FxHashMap};

/// Merges overlapping communities based on node co-occurrence similarity
pub fn merge_overlapping_communities(
    partitions: &[Partition],
    similarity_threshold: f64,
) -> Partition {
    // Start with the best partition
    let final_partition = partitions[0].clone();
    
    // Create a mapping of communities across partitions
    let mut community_overlap: FxHashMap<(CommunityId, CommunityId), usize> = FxHashMap::default();
    
    // Count node co-occurrences for each community pair
    for partition in partitions {
        for (&node, &comm) in partition {
            for (&other_node, &other_comm) in partition {
                if node != other_node && comm == other_comm {
                    for final_comm in partitions[0].values() {
                        if partitions[0][&node] == *final_comm {
                            let key = (comm, *final_comm);
                            *community_overlap.entry(key).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }
    
    // Merge communities with high overlap
    let mut merged_communities: FxHashMap<CommunityId, CommunityId> = FxHashMap::default();
    for ((comm1, comm2), overlap) in community_overlap {
        let max_size = partitions.iter()
            .map(|p| p.values().filter(|&c| *c == comm1 || *c == comm2).count())
            .max()
            .unwrap_or(1);
            
        let similarity = overlap as f64 / max_size as f64;
        if similarity > similarity_threshold {
            merged_communities.insert(comm2, comm1);
        }
    }
        
    final_partition
}

pub fn refinement(
    mut partition: Partition,
    archive: &[Solution],
    graph: &Graph,
    key_nodes: &FxHashSet<NodeId>,
    resolution: f64, // Higher values = more, smaller communities
    merge_threshold: Option<f64>, // Optional threshold for community merging
) -> Partition {

    // Calculate adaptive consensus if min_consensus is too low
    let min_consensus: usize = 1;
    let adaptive_consensus = (archive.len() as f64 * 0.2).ceil() as usize;
    let min_consensus = std::cmp::max(min_consensus, adaptive_consensus.min(3));

    // Create a node stability map to weight decisions
    let mut node_stability: FxHashMap<NodeId, f64> = FxHashMap::default();
    for &node in graph.nodes.iter() {  // Fixed: use iter() instead of keys()
        // Calculate how consistently this node belongs to the same community across solutions
        let mut community_counts: FxHashMap<CommunityId, usize> = FxHashMap::default();
        for sol in archive {
            let comm = sol.partition[&node];
            *community_counts.entry(comm).or_insert(0) += 1;
        }
        
        // Highest frequency divided by total solutions = stability score
        let max_count = community_counts.values().max().unwrap_or(&0);
        node_stability.insert(node, *max_count as f64 / archive.len() as f64);
    }

    // Continue iterating until no nodes change communities
    let mut changed = true;
    while changed {
        changed = false;
        // Process each key node and its neighbors
        for &k in key_nodes {
            let key_comm = partition[&k];
            for &n in graph.neighbors(&k) {
                // Skip if already in the key node's community
                if partition[&n] == key_comm {
                    continue;
                }
                
                // Count archive consensus with quality weighting
                let archive_consensus = archive.iter()
                    .filter(|sol| sol.partition[&k] == sol.partition[&n])
                    .count();
                
                // Vote count from the current neighborhood of n
                let (votes_for_key, votes_for_current) = graph.neighbors(&n)
                    .iter()
                    .fold((0.0, 0.0), |(mut key_votes, mut curr_votes), &neighbor| {
                        if partition[&neighbor] == key_comm {
                            key_votes += 1.0 * node_stability[&neighbor];
                        } else if partition[&neighbor] == partition[&n] {
                            curr_votes += resolution * node_stability[&neighbor];
                        }
                        (key_votes, curr_votes)
                    });
                
                // Reassign n to the key node's community if consensus meets threshold
                // and if weighted votes favor key community
                if archive_consensus >= min_consensus && votes_for_key > votes_for_current {
                    partition.insert(n, key_comm);
                    changed = true;
                }
            }
        }
    }
    
    // Merge overlapping communities if a threshold is provided
    if let Some(threshold) = merge_threshold {
        // Extract partition from each solution
        let partitions: Vec<Partition> = archive.iter()
            .map(|sol| sol.partition.clone())
            .collect();
        
        if !partitions.is_empty() {
            partition = merge_overlapping_communities(&partitions, threshold);
        }
    }
    
    partition
}

pub fn max_q_selection(archive: &[Solution]) -> &Solution {
    archive
        .iter()
        .max_by(|a, b| {
            let q_a = 1.0 - a.objectives[0] - a.objectives[1]; // 1 - intra - inter
            let q_b = 1.0 - b.objectives[0] - b.objectives[1];
            q_a.partial_cmp(&q_b).unwrap()
        })
        .unwrap()
}