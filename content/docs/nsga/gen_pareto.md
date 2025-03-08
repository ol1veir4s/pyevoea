+++
Title = "__generate_pareto_front()__"
weight = 18
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

In this post, we will show you how to use the `generate_pareto_front()` function from `MocdNsgaII` class. This function returns a list of candidate community partitions along with their corresponding objective values, giving you a set of non-dominated solutions (the Pareto front).

### Step-by-Step Example

1. **Create a NetworkX Graph**

   First, create a graph using NetworkX. For example, you might use the famous Karate Club graph:

   ```python
   import networkx as nx

   # Create the Karate Club graph
   G = nx.karate_club_graph()
   ```

2. **Initialize the Community Detection Instance**

   Next, import the module and create an instance of the NSGA-II-based community detection class. This instance requires the graph and several parameters such as population size, number of generations, and the rates for crossover and mutation.

   ```python
   import pyevoea
   # Initialize the community detection algorithm
   detector = pyevoea.MocdNsgaII(
       graph=G,           # your NetworkX graph
       debug_level=1,     # set to 1 or mroe for verbose logging
       pop_size=100,      # population size for the algorithm
       num_gens=500,      # number of generations
       cross_rate=0.8,    # crossover rate
       mut_rate=0.2       # mutation rate
   )
   ```

3. **Generate the Pareto Front**

   Call the `generate_pareto_front` method to obtain the set of candidate solutions. Each candidate is a tuple containing a partition (community assignment) and a list of objective values.

   ```python
   # Generate the Pareto front of community partitions
   pareto_front = detector.generate_pareto_front()

   # Print out the partitions and their objective values
   for partition, objectives in pareto_front:
       print("Partition:", partition)
       print("Objectives:", objectives)
   ```

{{< tip >}}
Expected output:

```sample_text
Partition: {0: 0, 1: 0, 2: 1, 3: 0, 4: 2, 5: 2, 6: 2, 7: 0, 8: 3, 9: 1, 10: 2, 11: 4, 12: 0, 13: 0, 14: 5, 15: 5, 16: 2, 17: 0, 18: 5, 19: 0, 20: 5, 21: 0, 22: 5, 23: 5, 24: 6, 25: 6, 26: 5, 27: 7, 28: 1, 29: 5, 30: 3, 31: 6, 32: 5, 33: 5}
Objectives: [0.39743589743589747, 0.2310979618671927]

...

Partition: {0: 0, 1: 0, 2: 0, 3: 0, 4: 1, 5: 1, 6: 1, 7: 0, 8: 2, 9: 3, 10: 1, 11: 0, 12: 0, 13: 0, 14: 2, 15: 2, 16: 1, 17: 0, 18: 2, 19: 0, 20: 2, 21: 0, 22: 2, 23: 2, 24: 2, 25: 2, 26: 2, 27: 2, 28: 2, 29: 2, 30: 2, 31: 2, 32: 2, 33: 2}
Objectives: [0.1923076923076923, 0.40861275476660086]


```
{{< /tip >}}

The `generate_pareto_front` function runs the multi-objective NSGA-II optimization process over your graph and returns the first Pareto front. The Pareto front consists of community partitions that are non-dominated according to the two objective functions:

These values collectively measure how densely connected the communities are internally and how sparse the connections are externally. The resulting set allows you to explore different trade-offs in community structure.




