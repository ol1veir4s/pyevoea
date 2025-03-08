+++
Title = "MocdNsgaII"
weight = 25
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

In this approach, each candidate solution (or **individual**) represents a possible partitioning of a network into communities. Unlike traditional single-objective methods that optimize a single measure (for example, modularity), this algorithm tackles community detection as a **multi-objective optimization** problem. It evaluates each solution using two conflicting but complementary objectives that capture different aspects of community structure.

---

### NSGA-II Algorithm Implementation

The implementation follows the standard steps of NSGA-II (Non-dominated Sorting Genetic Algorithm II):

1. **Initialization:**  
   A population of candidate solutions is generated. Each individual encodes a community partition using a locus-based adjacency representation. This means every node in the network is assigned a gene that points to one of its adjacent nodes, effectively grouping connected nodes together.

2. **Fitness Evaluation:**  
   Each individual is evaluated using two objective functions:
   - **Intra-objective:** Measures the internal connectivity (or “intra-link strength”) of communities.  
   - **Inter-objective:** Measures the external connectivity (or “inter-link strength”) between communities.  

   In the article, these objectives are derived from the modularity measure. The modularity \( Q \) can be written as:
   $$
   Q(C) = \sum_{c \in C} \left[\frac{|E(c)|}{m} - \left(\frac{\sum_{v \in c} \text{deg}(v)}{2m}\right)^2\right],
   $$
   where \( |E(c)| \) is the number of edges inside community \( c \), \( m \) is the total number of edges in the network, and \( \text{deg}(v) \) is the degree of node \( v \).  
   To turn this into two minimization problems, the first term is transformed as:
   $$
   \text{intra}(C) = 1 - \sum_{c \in C}\frac{|E(c)|}{m},
   $$
   and the second term is used directly as the inter objective:
   $$
   \text{inter}(C) = \sum_{c \in C}\left(\frac{\sum_{v \in c}\text{deg}(v)}{2m}\right)^2.
   $$
   The overall fitness can be seen as a trade-off:
   $$
   Q(C) = 1 - \text{intra}(C) - \text{inter}(C).
   $$

3. **Non-dominated Sorting and Crowding Distance:**  
   The algorithm uses a fast non-dominated sorting procedure to classify individuals into different Pareto fronts based on their objective values. Within the same front, a crowding distance is calculated to preserve diversity among solutions. This sorting is crucial because it helps the algorithm maintain a set of solutions that are optimal with respect to both objectives.

4. **Reproduction:**  
   Using genetic operators (such as uniform two-point crossover and mutation), offspring are generated from the current population. The reproduction step uses tournament selection (with a specified tournament size) to choose parents, ensuring that individuals with better fitness and diversity are more likely to be selected.

5. **Environmental Selection and Convergence:**  
   The parent and offspring populations are combined, and the algorithm re-applies the non-dominated sort and crowding distance calculations to select the top solutions up to a predefined population size. The best fitness in the population is monitored for convergence. If there is no significant improvement over a number of generations, the evolution stops.

6. **Output:**  
   Finally, the algorithm extracts the Pareto front (first front) of non-dominated solutions. Additionally, model selection methods can be applied (such as choosing the solution with maximum modularity or using a Max–Min Distance approach) to recommend a single partition for decision makers.

---

### Optimization

**A single-objective optimization (e.g., maximizing modularity) may miss important community structures, especially in networks with hierarchical or overlapping communities.** By splitting the modularity measure into two complementary objectives, the algorithm is able to:

- **Balance competing aspects:**  
  Maximizing internal connectivity (keeping communities dense) and minimizing external connectivity (keeping communities well-separated) are inherently conflicting. The algorithm must balance these, which is achieved through multi-objective optimization.

- **Reveal multiple resolutions:**  
  Instead of converging to a single solution, the multi-objective approach produces a set of candidate partitions (the Pareto front). These solutions can show different levels of granularity in the community structure.

- **Improve robustness:**  
  The evolutionary process with NSGA-II—using non-dominated sorting and crowding distance—helps avoid premature convergence to local optima. This allows the algorithm to explore a wider solution space and find more robust community partitions.

In summary, the optimization strategy is built on the idea that the best community structure is not necessarily the one that maximizes a single metric. By considering two complementary objectives:
$$
\text{intra}(C) = 1 - \sum_{c \in C}\frac{|E(c)|}{m} \quad \text{and} \quad \text{inter}(C) = \sum_{c \in C}\left(\frac{\sum_{v \in c}\text{deg}(v)}{2m}\right)^2,
$$
the algorithm can navigate the trade-offs inherent in network community detection. The overall modularity can then be seen as:
$$
Q(C) = 1 - \text{intra}(C) - \text{inter}(C),
$$
which naturally leads to selecting partitions that represent the best compromise between internal density and external separation.
