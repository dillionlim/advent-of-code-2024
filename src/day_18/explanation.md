# Day 18: RAM Run

## Input Bounds

Based on preliminary data analysis, all the numbers are within the range of 0 to 70.

## Part A

### Abridged Problem Statement

Given an input array list of $(x, y)$ pairs, where the first 1024 pairs of coordinates in this array list refer to the obstacles, find the shortest path from the top-left corner to the bottom-right corner.

### Solution

Simply run a level-based BFS search on the grid. We increment the count of the level for each set of cells with distance $x$ from the starting point. Effectively, we pop all the cells in the queue at the beginning of each iteration. Once we find the bottom-right corner, we return the count of the level at that point.

### Code Complexity

**Time Complexity:** $O(N \times M)$

* $N$ is the number of rows in the grid.
* $M$ is the number of columns in the grid.

In the worst case, we need to iterate through all cells in the grid.

**Additional Space Complexity:** $O(N \times M)$

The grid needs to be stored. Furthermore, we maintain a visited array to keep track of visited cells.

**Final answer:** 140.

## Part B

### Abridged Problem Statement

Given that the obstacle in line $i$ is placed at time $t$, find the coordinates of the obstacle corresponding to the earliest time $t$ such that there is no valid path from the top-left corner to the bottom-right corner.

### Solution

We will use a disjoint set union on-the-fly algorithm.

After all obstacles are placed, we know that there will not be any valid path from the top-left corner to the bottom-right corner. Therefore, the grid can be partitioned into various connected components, where the top-left corner and the bottom-right corner are in different connected components. We build a disjoint set union on this state of the grid.

Thereafter, we will iterate the obstacle list in reverse, and remove the obstacle. We will then unite this cell with its neighboring cells. Afterwards, we will check if there is a valid path from the top-left corner to the bottom-right corner by checking if the top-left corner and bottom-right corner are in the same connected component. We update the first set of coordinates where the top-left corner and bottom-right corner are in the same connected component.

#### Disjoint Set Union

We need to implement two functions for the disjoint set union (DSU):

* `get(x)`: Find the root of the set containing the element `x`.
* `unite(x, y)`: Unite the sets containing the elements `x` and `y`.

To implement this, we will store the sets as trees, where the root is the representative of the set.

First, we will initialize each element in the DSU to be its own set. We will set the convention that the value in the DSU array is:

* **The index of its parent** if the element is not the root.
* **The size of the set** if the element is the root. 

To distinguish between the root and non-root elements, we will set the value of the elements corresponding to roots as negative values. That is, root elements will have a negative value in the DSU array, with $-x$ representing a size of $x$.

#### Path Compression

The naive implementation of the `get(x)` function involves recursively finding the parent of the element until we reach a root element. However, this implementation leads to a $O(N)$ time complexity in the worst case if the tree degenerates into long chains. Therefore, we make an observation that we can simply point each of the children elements to the root of the tree. 

We note that if we call `get(x)` for some vertex $x$, we actually find the root $r$ for all vertices on the path between $x$ and the root $r$. We will reduce the length of the paths for all those nodes by setting the parent of each of these vertices directly to $r$. For example, calling `get(7)` below will shorten the path for the visited nodes 2, 3, 5 and 7.

<figure>
    <img src="https://cp-algorithms.com/data_structures/DSU_path_compression.png"
         alt="path_compression">
    <figcaption><i>Source: <a href="https://cp-algorithms.com/data_structures/disjoint_set_union.html">Disjoint Set Union, CP Algorithms</a></i></figcaption>
</figure>

#### Union by Size

We will use union by size to merge determine which tree gets attached to the other. In the naive implementation, the second tree being always attached to the first tree can lead to chains of length $O(N)$. Therefore, we will attach the tree with a smaller size to the tree of a bigger size. 

This is why we need to store the size of the tree in the root of the tree.

If we use both optimizations (path compression, union by size), we will get a time complexity of $O(\alpha(N))$, where $\alpha(n)$ is the inverse Ackermann function, which grows very slowly.

### Code Complexity

**Time Complexity:** $O(N \times M + K \times \alpha(N\times M))$

* $N$ is the number of rows in the grid.
* $M$ is the number of columns in the grid.
* $K$ is the number of obstacles.
* $\alpha(n)$ is the inverse Ackermann function.

We will first need to iterate through the whole grid to construct the initial disjoint set union. 

Then, for each obstacle, we will perform the union operation on its neighbouring cells, which takes $O(\alpha(N\times M))$ time for each union operation, and since there will be at most 4 union operations done, it will take $O(\alpha(N\times M))$ time for the unions required for each cell. Afterwards, we will check if there is a valid path, which also will take $O(\alpha(N\times M))$ time.

This is given that we use both path compression and union by size or rank.

**Additional Space Complexity:** $O(N \times M)$

The grid needs to be stored, and the disjoint set union also needs to be stored.

**Final answer:** 65,6.
