+++
Title = "Non-Dominated Sorting Genetic Algorithm 2"
weight = 5
+++

## Introduction

In this guide, we'll explore how to use the `nsga_ii` function for community detection on a NetworkX graph. The NSGA-II algorithm employs a multi-objective approach to identify community structures within networks.

## Prerequisites

Ensure that you have installed the necessary libraries and the module:

```bash
pip list | grep re-mocd
```

If you're using a unix-based system, you should see:


> re_mocd         0.2.3       


## Step 1: Prepare Your Graph

We will use a sample graph for demonstration purposes. Here, we use the Karate Club graph:

```python
import networkx as nx

# Create a sample graph
G = nx.karate_club_graph()
```

## Step 2: Run the nsga_ii Algorithm

Import and run the `nsga_ii` function on the graph, optionally specifying a debug level:

```python
from re_mocd import nsga_ii

# Run the algorithm with a debug level of 1. 0 is the default.
communities = nsga_ii(G, debug=1)
```

## Step 3: Inspect the Results

The output is a dictionary mapping node IDs to community IDs. You can print the results as follows:

```python
print("Detected communities:")
for node, comm in communities.items():
    print(f"Node {node}: Community {comm}")
```