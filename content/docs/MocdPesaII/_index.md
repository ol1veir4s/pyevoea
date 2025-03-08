+++
Title = "MocdPesaII"
weight = 35
+++

<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/KaTeX/0.16.4/katex.min.css">
<script defer src="https://cdnjs.cloudflare.com/ajax/libs/KaTeX/0.16.4/katex.min.js"></script>
<script defer src="https://cdnjs.cloudflare.com/ajax/libs/KaTeX/0.16.4/contrib/auto-render.min.js"></script>
<script>
    document.addEventListener("DOMContentLoaded", function() {
        renderMathInElement(document.body, {
            delimiters: [
                { left: "$$", right: "$$", display: true },
                { left: "$", right: "$", display: false }
            ]
        });
    });
</script>

In this post, we explore the Pareto Envelope-based Selection Algorithm II (PESA-II) and how it is applied for multi-objective community detection. We’ll discuss the underlying equations, the evolutionary process, and the unique components of the algorithm—such as the hypergrid archive—that make it effective at balancing conflicting objectives. If you’re familiar with NSGA-II, you’ll notice some differences in the way PESA-II maintains diversity and selects candidate solutions.

---

## Overview

Community detection in complex networks is a challenging problem because good partitions must balance two conflicting properties:
- **Intra-community connectivity:** Communities should be densely connected internally.
- **Inter-community separation:** Communities should be sparsely connected with the rest of the network.

Traditionally, these aspects are captured by the *modularity* measure \( Q \). In our multi-objective framework, however, we break modularity into two separate objectives:

1. **Intra-objective:** Maximizes the fraction of edges inside communities.
2. **Inter-objective:** Minimizes the expected inter-community edge strength.

These objectives are defined as:

$$
\text{intra}(C) = 1 - \sum_{c \in C} \frac{|E(c)|}{m},
$$

$$
\text{inter}(C) = \sum_{c \in C} \left(\frac{\sum_{v \in c} \text{deg}(v)}{2m}\right)^2,
$$

where:  
- \( C \) is a partition of the network,  
- \( |E(c)| \) is the number of edges within community \( c \),  
- \( m \) is the total number of edges, and  
- \( \text{deg}(v) \) is the degree of node \( v \).

These lead to an aggregated formulation of modularity:

$$
Q(C) = 1 - \text{intra}(C) - \text{inter}(C).
$$

Unlike single-objective approaches that optimize \( Q \) directly, the multi-objective method seeks a set of candidate solutions that reflect different trade-offs between these two objectives.

---

## How PESA-II Works

PESA-II is a multi-objective evolutionary algorithm (MOEA) designed to effectively handle complex optimization tasks. Let’s break down its key components:

### 1. Population Initialization

- **Candidate Representation:**  
  Each candidate solution (or individual) is a community partition of the network. The algorithm uses a locus-based adjacency encoding where each node is assigned a gene that points to one of its adjacent nodes. This representation naturally leads to connected subgraphs that form communities.

- **Initial Population:**  
  A population of candidate solutions is randomly generated. This diversity is crucial for exploring the solution space effectively.

### 2. Evolutionary Phase

PESA-II evolves the population through standard genetic operators (crossover and mutation) while incorporating a unique archive-based mechanism to maintain diversity:

- **Reproduction and Variation:**  
  - **Crossover:** The algorithm employs a uniform two-point crossover to mix genetic material between parent solutions without bias.
  - **Mutation:** Random mutation is applied to introduce variability, ensuring that the search does not get trapped in local optima.

- **Evaluation:**  
  Each candidate is evaluated using the two objectives (\(\text{intra}\) and \(\text{inter}\)). The computational cost for evaluating an individual is \( O(m + n) \) where \( m \) is the number of edges and \( n \) is the number of nodes.

### 3. Archive Maintenance with a Hypergrid

One of the distinguishing features of PESA-II is its use of a hypergrid for archive maintenance:

- **Hypergrid Structure:**  
  Instead of relying solely on non-dominated sorting (as in NSGA-II), PESA-II divides the objective space into a grid of “niches.” Each niche can hold candidate solutions. The algorithm keeps track of the density in each niche, favoring solutions that lie in less crowded areas of the objective space.

- **Diversity Preservation:**  
  This niche-based selection ensures that the archive (or external population) of non-dominated solutions is well spread out along the Pareto front. Maintaining diversity is essential for capturing multiple levels of resolution in community structure.

### 4. Environmental Selection and Model Selection

After generating a combined population of parents and offspring, PESA-II selects the next generation based on two factors:
- **Non-Dominated Ranking:** Candidates are classified based on Pareto dominance.
- **Niche Density:** Candidates in less crowded hypergrid cells are preferred.

This process is repeated over a number of generations until the algorithm converges or the maximum number of generations is reached.

#### Model Selection Methods

The PESA-II implementation discussed here offers two strategies for selecting a single, representative community partition from the Pareto front:

- **max_q:**  
  This method selects the candidate with the highest aggregated modularity score:
  
  $$
  Q(C) = 1 - \text{intra}(C) - \text{inter}(C).
  $$

- **mini_max:**  
  The mini_max method goes one step further by comparing the Pareto front from the real network with those obtained from random networks of the same scale. The distance between solutions is measured by the Euclidean distance between objective values:
  
  $$
  dis(C, C') = \sqrt{(\text{intra}(C) - \text{intra}(C'))^2 + (\text{inter}(C) - \text{inter}(C'))^2}.
  $$
  
  The candidate solution that maximizes the minimum distance to any solution from the random networks is selected:
  
  $$

  \text{Max–Min} = \arg\max_{C \in SF} \left\{\min_{C' \in CF} \, dis(C, C')\right\},
  
  $$
  
  where \( SF \) is the Pareto front of the real network and \( CF \) is the collection of Pareto fronts from random networks. This selection highlights the solution with the most statistically significant community structure.