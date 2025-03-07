+++
Title = "Visualizing Communities Detected"
weight = 1
+++

## Introduction

This tutorial demonstrates how to generate and visualize a graph with community structures detected using the `NSGA-II` algorithm. We will:

1. Generate a synthetic graph using the **LFR benchmark model**.
2. Detect communities using **NSGA-II-based detection from re_mocd**.
3. Use **NetworkX** and **Seaborn** to visualize the communities with distinct colors.

## Prerequisites

Ensure you have the required libraries installed:

```bash
pip install networkx matplotlib seaborn re_mocd
```

## Step 1: Generate a Graph with Community Structure

We use the **LFR benchmark graph** to create a synthetic network with realistic community structures.

```python
import networkx as nx
import matplotlib.pyplot as plt
import seaborn as sns
import re_mocd 
from networkx.generators.community import LFR_benchmark_graph

# Set a clean Seaborn theme
sns.set_theme(style="white")

# LFR benchmark parameters
n = 100  # Number of nodes
tau1 = 3  # Degree distribution exponent
tau2 = 1.5  # Community size distribution exponent
mu = 0.1  # Mixing parameter (controls inter-community edges)

# Generate the LFR benchmark graph
G = LFR_benchmark_graph(n, tau1, tau2, mu, average_degree=5, min_community=30)
```

## Step 2: Detect Communities

```python
# Run the NSGA-II community detection algorithm
communities = re_mocd.nsga_ii(G)
```

## Step 3: Visualize the Detected Communities

To enhance visualization, we assign distinct colors to each detected community and display the graph.

```python
# Get unique community IDs
unique_comms = set(communities.values())

# Generate a color palette for communities
palette = sns.color_palette("hsv", len(unique_comms))
comm_color_map = {comm: palette[i] for i, comm in enumerate(unique_comms)}

# Assign colors to nodes
node_colors = [comm_color_map[communities[node]] for node in G.nodes()]
```

Plot using `matplotlib.pyplot`:

```
pos = nx.spring_layout(G, seed=42)

plt.figure(figsize=(16, 12))

nx.draw_networkx_edges(G, pos, edge_color='gray', alpha=0.7, width=2)

nx.draw_networkx_nodes(G, pos, node_color=node_colors, edgecolors='black', linewidths=1.5, node_size=600)
plt.axis('off')
plt.show()
```

## Output

...
{{< picture "index.png" "index.png" "Image alt text" >}}
...
