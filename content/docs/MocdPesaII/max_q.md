+++
Title = "__max_q()__"
weight = 50
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

In this post we focus on the `max_q` function from the PESA-II implementation. This function selects the best community partition from the Pareto front by choosing the one with the highest modularity score \( Q \), where modularity is given by:

$$
Q(C) = 1 - \text{intra}(C) - \text{inter}(C).
$$

### Step-by-Step Example

1. **Create a NetworkX Graph**

   As before, begin by creating a NetworkX graph. We again use the Karate Club graph:

   ```python
   import networkx as nx

   # Create the Karate Club graph
   G = nx.karate_club_graph()
   ```

2. **Initialize the PESA-II Community Detection Instance**

   Import the `pyevoea` module and create an instance with appropriate parameters:

   ```python
   import pyevoea

   # Initialize the PESA-II based community detection algorithm
   detector = pyevoea.MocdPesaII(
       graph=G,
       debug_level=1,
       rand_networks=3,
       pop_size=100,
       num_gens=500,
       cross_rate=0.8,
       mut_rate=0.2
   )
   ```

3. **Select the Best Partition Using max_q**

   Use the `max_q` method to obtain the recommended community partition based on the maximum modularity score. This function internally computes \( Q(C) \) for each candidate and returns the best one.

   ```python
   # Get the recommended partition with maximum modularity Q
   best_partition = detector.max_q()

   # Display the selected partition
   print("Recommended Partition:", best_partition)
   ```

The `max_q` function works by:

- Running the evolutionary process to produce an archive (Pareto front) of candidate solutions.
- Computing the modularity \( Q(C) \) for each candidate as:
  
  $$
  Q(C) = 1 - \text{intra}(C) - \text{inter}(C).
  $$

- Selecting the candidate with the highest \( Q \). This selection method helps in balancing internal connectivity and external separation to yield a community structure that is both dense within communities and sparse between them.

This recommended partition provides a single, aggregated solution that you can use for further analysis or visualization.