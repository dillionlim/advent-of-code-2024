# Day 16: Reindeer Maze

## Input Bounds

Based on preliminary data analysis, the input is simply a grid with 'S', 'E', '.' and '#' characters.

## Part A

### Abridged Problem Statement

Given a grid with two points labelled S and E, find the shortest path from S to E. Moving straight has a cost of 1, and turning either clockwise or anticlockwise has a cost of 1000.

### Solution

Simply use Dijkstra's algorithm to find the shortest path on the grid. Turning and moving has a cost of 1001, while moving in the completely opposite direction has a cost of 2001. Moving straight ahead has a cost of 1. 

We can do an early return once we encounter the endpoint in our priority queue.

### Code Complexity

**Time Complexity:** $O(NM \log NM + NM)$

* $N$ is the number of rows in the grid.
* $M$ is the number of columns in the grid.

In the worst case, there will be $O(NM)$ vertices and edges. However, in practice, this runs much faster.

The time complexity of Dijkstra's algorithm with a priority queue is $O(E + V\log V)$.

**Additional Space Complexity:** $O(N \times M)$

The grid needs to be stored. Furthermore, in the worst case, all points can be pushed to the priority queue.

**Final answer:** 94444.

## Part B

### Abridged Problem Statement

Find the total number of unique tiles that are part of a shortest path (as found in part A). There may be multiple shortest paths.

### Solution

We store all the predecessors of a node when performing Dijkstra's algorithm.

* When we encounter a path with lower cost, we reset the predecessor list.
* When we encounter a path of equal cost, we will append to the predecessor's list.

We need to maintain the direction in addition to the coordinates of the point.

After performing Dijkstra's algorithm, we can simply run a DFS on the predecessor's list, starting from the ending node. We add all points into a hashset, and at the end, we simply return the length of the hashset.

### Code Complexity

**Time Complexity:** $O(NM \log NM + NM)$

* $N$ is the number of rows in the grid.
* $M$ is the number of columns in the grid.

In the worst case, there will be $O(NM)$ vertices and edges. However, in practice, this runs much faster.

The time complexity of Dijkstra's algorithm with a priority queue is $O(E + V\log V)$.

Dijkstra's algorithm dominates the DFS conducted, since the DFS has a worst case time complexity of $O(NM)$.

**Additional Space Complexity:** $O(N \times M)$

The grid needs to be stored. Furthermore, in the worst case, all points can be pushed to the priority queue.

**Final answer:** 502.