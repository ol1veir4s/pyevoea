+++
Title = "Visualizing the Pareto Front"
weight = 105
+++

In this tutorial, we’ll show you how to visualize the Pareto front produced by the community detection algorithm. The Pareto front is a set of candidate solutions that trade off the two objectives—usually the intra-community connectivity and the inter-community separation. By plotting these objective values, you can visually assess the trade-offs between different solutions.

### Step-by-Step Guide

1. **Generate the Pareto Front**

   First, run your community detection algorithm to generate the Pareto front. For example, if you are using our PESA-II or NSGA-II implementation, you might call:
   
   ```python
   # Generate Pareto front (each candidate is a tuple: (partition, [intra, inter]))
   pareto_front = detector.generate_pareto_front()
   ```

2. **Extract Objective Values**

   Next, extract the objective values from each candidate. Here we assume that each candidate’s objectives is a list or tuple with two values (e.g., intra and inter).
   
   ```python
   # Extract objective values from each candidate solution
   intra_values = [obj[0] for (_, obj) in pareto_front]
   inter_values = [obj[1] for (_, obj) in pareto_front]
   ```

3. **Plot Using Matplotlib**

   Use Matplotlib to create a scatter plot where the x-axis corresponds to the intra objective and the y-axis corresponds to the inter objective.
   
   ```python
   import matplotlib.pyplot as plt

   plt.figure(figsize=(8, 6))
   plt.scatter(intra_values, inter_values, color='blue', alpha=0.7)
   plt.xlabel("Intra Objective Value")
   plt.ylabel("Inter Objective Value")
   plt.title("Pareto Front of Candidate Community Solutions")
   plt.grid(True)
   plt.show()
   ```

### Full Code Example

```python
import networkx as nx
import matplotlib.pyplot as plt
import pyevoea  # Replace with your module name

# 1. Create a graph (using the Karate Club graph as an example)
G = nx.karate_club_graph()

# 2. Initialize the community detection algorithm
detector = pyevoea.MocdPesaII(
    graph=G,
    debug_level=1,
    rand_networks=3,
    pop_size=100,
    num_gens=500,
    cross_rate=0.8,
    mut_rate=0.2
)

# 3. Generate the Pareto front
pareto_front = detector.generate_pareto_front()

# 4. Extract objective values (assume objectives: [intra, inter])
intra_values = [obj[0] for (_, obj) in pareto_front]
inter_values = [obj[1] for (_, obj) in pareto_front]

# 5. Plot the Pareto front
plt.figure(figsize=(8, 6))
plt.scatter(intra_values, inter_values, color='blue', alpha=0.7)
plt.xlabel("Intra Objective Value")
plt.ylabel("Inter Objective Value")
plt.title("Pareto Front of Candidate Community Solutions")
plt.grid(True)
plt.show()
```

By following these steps, you’ll have a clear scatter plot that shows the trade-offs between the two objectives, helping you visualize the landscape of candidate solutions.