+++
Title = "__min_max()__"
weight = 60
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


In this post, we’ll walk through using the `min_max` function. This method goes a step further than simply picking the partition with the highest modularity. Instead, it compares the Pareto front from your actual network with Pareto fronts generated from a set of random networks (of the same scale) and selects the community partition that deviates the most from what you would expect by chance.

### Step-by-Step Example

1. **Create a NetworkX Graph**

   Begin by creating a graph using NetworkX. For instance, you can use the popular Karate Club graph:

   ```python
   import networkx as nx

   # Create the Karate Club graph
   G = nx.karate_club_graph()
   ```

2. **Initialize the PESA-II Community Detection Instance**

   Next, import the `pyevoea` module and create an instance. In addition to the parameters used for evolution (population size, number of generations, crossover and mutation rates), you also specify `rand_networks` — the number of random networks to generate for the model selection process.

   ```python
   import pyevoea

   # Initialize the PESA-II community detection algorithm
   detector = pyevoea.MocdPesaII(
       graph=G,           # Pass the NetworkX graph
       debug_level=1,       # Set debug level for logging
       rand_networks=3,     # Number of random networks for model selection
       pop_size=100,        # Population size
       num_gens=500,        # Number of generations
       cross_rate=0.8,      # Crossover rate
       mut_rate=0.2         # Mutation rate
   )
   ```

3. **Select the Best Partition Using mini_max**

   The `mini_max` function performs the following:
   - Runs the evolutionary phase to get the Pareto front for your real network.
   - Generates several random networks (using the same scale as your original graph).
   - Evolves each random network to get their corresponding Pareto fronts.
   - Compares the solutions from the real network with those from the random networks.
   - Selects the candidate solution that has the largest deviation from the random controls, indicating the most statistically significant community structure.

   Here’s how you call `min_max`:

   ```python
   # Get the recommended partition using the min_max selection strategy
   recommended_partition = detector.min_max()

   # Print the resulting partition
   print("Recommended Partition (min_max):", recommended_partition)
   ```

The `min_max` function uses a model selection method designed to determine which community structure is most different from randomness. The process involves:

- **Real Pareto Front Generation:**  
  The algorithm evolves the real network to create a set of candidate solutions (a Pareto front).

- **Random Networks Comparison:**  
  It then generates a number of random networks that have similar properties (e.g., same number of nodes and edges) as the real network. For each random network, a Pareto front is computed using the same evolutionary procedure.

- **Deviation Measurement:**  
  For each solution in the real Pareto front, the function calculates its distance to the closest solution in each random Pareto front. The distance is measured using the Euclidean distance between objective values:
  $$
  dis(C, C') = \sqrt{(intra(C) - intra(C'))^2 + (inter(C) - inter(C'))^2}.
  $$