# Day 10: Hoof It

## Input Bounds

From preliminary data analysis, we simply have a grid of numeric characters.

## Part A

### Abridged Problem Statement

You are given a grid with numbers from 0 to 9. Find the total number of unique pairs of 0 and 9s in the grid, such that there exists a path between them such that every successive number increases by 1. That is, the path should be 0123456789. 

### Solution

For every 0 in the grid, run a BFS search to find all such paths. Use a `visited` array to avoid double-counting pairs, as there may be multiple paths that lead to the same endpoint.

### Code Complexity

**Time Complexity:** $O(k \times N \times M)$

* $k$ is the number of 0 cells in the grid
* $N$ is the number of rows in the grid
* $M$ is the number of columns in the grid

Each BFS for each 0 cell takes $O(N \times M)$ time in the worst case.

**Additional Space Complexity:** $O(N \times M)$

The `visited` array and the grid both require $O(N \times M)$ space.

**Final answer:**

## Part B

### Abridged Problem Statement

You are given a grid with numbers from 0 to 9. Find the total number of unique paths between 0 and 9, such that there exists a path between them such that every successive number increases by 1. That is, the path should be 0123456789. 

### Solution

We can use an almost similar idea from part A. However, we note that we need to modify our `visited` array. In particular, we note that the number of ways to get to a cell $(x, y)$ is given by the sum of the total number of ways to get to its neighbouring cells.

That is, denoting $w_{x,y}$ as the number of ways to get to $(x,y)$, we can write 
$$w_{x,y} = w_{x-1,y} + w_{x,y-1} + w_{x+1,y} + w_{x,y+1}$$

Therefore, we simply need to use dynamic programming to sum up the number of ways to get to the ending cell, and sum that up for all pairs of (0, 9). We will instead use the `visited` array to store the total number of ways to get to a cell, and only push a cell to a queue if it has not been visited before (that is, the number of ways to get to it is currently 0).

### Code Complexity

**Time Complexity:** $O(k \times N \times M)$

* $k$ is the number of 0 cells in the grid
* $N$ is the number of rows in the grid
* $M$ is the number of columns in the grid

Each BFS for each 0 cell takes $O(N \times M)$ time in the worst case.

**Additional Space Complexity:** $O(N \times M)$

The `visited` array and the grid both require $O(N \times M)$ space.

**Final answer:**
