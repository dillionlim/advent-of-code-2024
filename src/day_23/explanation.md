# Day 23: LAN Party

## Input Bounds

From preliminary data analysis, all the inputs are lowercase two-letter strings, separated by a `-`.

## Part A

### Abridged Problem Statement

Given a series of edges between nodes which are identified by a two-letter string, construct the associated graph. Find the number of 3-cliques in this graph, where there exists at least one node which starts with `t`.

### Solution

First, we construct the adjacency list of the graph based on the input edges. At the same time, we construct the adjacency matrix of the graph.

We notice that we can hash the two-letter string as an integer base 26. That is, denoting the index of the first letter as $a$, and that of the second letter as $b$ (where `a = 0, b = 1,` and so on), we can hash these as $26a + b$.

Therefore, all pairs starting with `t` must be in the range of `ta`, which corresponds to $19 \times 26 + 0 = 494$ to `tz`, which corresponds to $19 \times 26 + 25 = 519$.

Therefore, we will set the first node in the 3-clique as a node starting with `t`. For this node, we iterate through all unique pairs of the node's neighbours in the adjacency list, and check if the pair of neighbours are adjacent to each other using the adjacency matrix. To avoid double-counting, we will use a visited array and ensure that pair of neighbours have not been visited. 

### Code Complexity

**Time Complexity:** $O(V^2)$

* $V$ is the number of vertices in the graph.

For each vertex, there will be at most $V-1$ neighbours. We iterate through a fixed number of starting nodes for the 3-clique (26 nodes). For each starting node, we visit all unique pairs of neighbours, of which there will be $O(V^2)$ pairs.

**Additional Space Complexity:** $O(V^2)$

For the purpose of using a static 2D array, the value of $V$ is a constant $26 \times 25 + 25 = 676$. This is the worst case scenario. The adjacency matrix needs to be constructed.

**Final answer:** 1238.

## Part B

### Abridged Problem Statement

Given a series of edges between nodes which are identified by a two-letter string, construct the associated graph. It is known that there is only one maximal clique in the graph (inferred from problem statement). Find the maximal clique in the graph, and return all nodes in the maximal clique, sorted in lexicographical order, comma-separated.

### Solution

While the most common algorithm for finding **all maximal cliques** in a graph is the [Bron-Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm), we realise that this problem statement hints at there only being 1 maximal clique in the graph. Therefore, this algorithm is overkill.

We can therefore use a simpler, greedy solution to find the maximal clique in the graph. We will similarly start off by constructing the adjacency list and adjacency matrix of the graph.

Then, for each node in the graph, we will iterate over its neighbours (since all nodes that are part of this clique must also be a neighbour of this node). Then, we will incrementally build the clique by adding one neighbour at a time, if it is a neighbour of all current nodes in the clique. If the clique is larger than the previously found largest clique, then we store this clique instead.

As an optimization, we note that all nodes that are previously found to be a part of a clique do not need to be revisited, as the same clique will be obtained.

### Code Complexity

**Time Complexity:** $O(V^2)$

* $V$ is the number of vertices in the graph.

For each vertex in the graph, we iterate through each neighbour of it to determine if it is part of its clique.

**Additional Space Complexity:** $O(V^2)$

For the purpose of using a static 2D array, the value of $V$ is a constant $26 \times 25 + 25 = 676$. This is the worst case scenario. The adjacency matrix needs to be constructed, which takes $O(V^2)$ space.

**Final answer:** bg,bl,ch,fn,fv,gd,jn,kk,lk,pv,rr,tb,vw.
