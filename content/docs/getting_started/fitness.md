+++
Title = "Checking fitness of a partition"
weight = 30
+++

## Introduction

This tutorial demonstrates how to get the score of a graph given a community partition using the `fitness` function. This score measures the quality of the community structure.

## Step 1: Prepare Your Graph and Partition

Start by generating a sample graph and creating a partition. In this example, we use the Karate Club graph and define a simple partition:

```python
import networkx as nx

# Create a sample graph
G = nx.karate_club_graph()

# Example partition: mapping node IDs to community IDs (this is just a simple example)
partition = {node: 0 if node < 10 else 1 for node in G.nodes()}
```

## Step 2: Calculate the Modularity Score

Import the `fitness` function from your module and compute the modularity score:

```python
from re_mocd import fitness

# Calculate the modularity score for the graph and partition
modularity_score = fitness(G, partition)
print("Score:", modularity_score)
```

Using the `fitness` function, you can evaluate the quality of a community partition by calculating its score. Experiment with different partitions based on your community detection results for further analysis.
