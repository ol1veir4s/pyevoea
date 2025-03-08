+++
Title = "__generate_pareto_front()__"
weight = 40
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

In this post we show how to use the `generate_pareto_front` function from the PESA-II implementation for multi-objective community detection. This function returns a set of candidate community partitions (the Pareto front) along with their corresponding objective values.

### Step-by-Step Example

1. **Create a NetworkX Graph**

   First, we create a NetworkX graph. In this example we use the Karate Club graph:

   ```python
   import networkx as nx

   # Create the Karate Club graph
   G = nx.karate_club_graph()
   ```

2. **Initialize the PESA-II Community Detection Instance**

   Next, import the `pyevoea` module and create an instance of the class. The instance requires parameters like population size, number of generations, and rates for crossover and mutation:

   ```python
   import pyevoea

   # Initialize the PESA-II community detection algorithm
   detector = pyevoea.MocdPesaII(
       graph=G,            # Pass the NetworkX graph
       debug_level=1,       # Set debug level for logging
       rand_networks=3,     # Number of random networks for model selection (if minimax selection needed)
       pop_size=100,        # Population size
       num_gens=500,        # Number of generations
       cross_rate=0.8,      # Crossover rate
       mut_rate=0.2         # Mutation rate
   )
   ```

3. **Generate the Pareto Front**

   Call the `generate_pareto_front` method to run the evolutionary phase and obtain the Pareto front. Each element in the returned list is a tuple with a partition (community assignment) and a list of objective values.

   ```python
   # Generate the Pareto front of candidate solutions
   pareto_front = detector.generate_pareto_front()

   # Display each partition with its objective values
   for partition, objectives in pareto_front:
       print("Partition:", partition)
       print("Objectives:", objectives)
   ```


The `generate_pareto_front` method in the PESA-II implementation carries out an evolutionary phase that includes:

- **Initialization and Reproduction:** A population of candidate solutions is generated using a locus-based encoding, and then evolved through crossover and mutation.
- **Pareto Archive Maintenance:** Instead of a single population, PESA-II maintains an archive of non-dominated solutions using a hypergrid structure to ensure diversity.
- **Evaluation:** Each solution is evaluated using two conflicting objectives, similar to those derived from the modularity measure:
  
  $$
  \text{intra}(C) = 1 - \sum_{c \in C}\frac{|E(c)|}{m} \quad \text{and} \quad \text{inter}(C) = \sum_{c \in C}\left(\frac{\sum_{v \in c}\text{deg}(v)}{2m}\right)^2.
  $$
