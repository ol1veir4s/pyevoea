+++
Title = "Pareto Envelope-Based Selection Algorithm 2"
weight = 10
+++

## Introduction

This tutorial demonstrates how to use both `pesa_ii_minimax` and `pesa_ii_maxq` functions to detect communities in a NetworkX graph. Both algorithms take a graph as input and return a mapping of node IDs to their detected community IDs. The `debug` parameter is available to control the verbosity of the output.

## Prerequisites

Before you begin, ensure you have installed the required libraries and the module containing these algorithms:

```bash
pip install your_module_name
```

## Step 1: Prepare Your Graph

For demonstration purposes, we use the classic Karate Club graph from NetworkX:

```python
import networkx as nx

# Create a sample graph (Karate Club graph)
G = nx.karate_club_graph()
```

## Step 2: Running pesa_ii_minimax

The `pesa_ii_minimax` function performs community detection using a minimax approach. Import the function from your module and run it on your graph:

```python
from your_module_name import pesa_ii_minimax

# Run the pesa_ii_minimax algorithm with a debug level of 2
communities_minimax = pesa_ii_minimax(G, debug=2)

print("Detected communities (minimax):")
for node, comm in communities_minimax.items():
    print(f"Node {node}: Community {comm}")
```

## Step 3: Running pesa_ii_maxq

Similarly, the `pesa_ii_maxq` function applies a maxq approach to community detection. Use the following code to run it:

```python
from your_module_name import pesa_ii_maxq

# Run the pesa_ii_maxq algorithm with a debug level of 2
communities_maxq = pesa_ii_maxq(G, debug=2)

print("Detected communities (maxq):")
for node, comm in communities_maxq.items():
    print(f"Node {node}: Community {comm}")
```