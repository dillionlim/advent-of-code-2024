# Day 20: Race Condition

## Input Bounds

From preliminary data analysis, the input is a grid with '#', '.', 'S' and 'E' characters.

## Part A

### Abridged Problem Statement

You are given a grid where 'S' is the start position, 'E' is the ending position, and '.' is empty space while '#' are walls. It is known that there is only one valid path from the start to the end. 

You can skip walls from an empty space **once** by moving at most two squares such that the ending square is an empty space. Let $x$ be the number of moves saved by performing the skip. Determine how many pairs of starting and ending skip squares there are such that the number of moves saved by performing the skip is more than or equal to 100.

### Solution

We first perform a BFS to record the index of each square, starting from 0 at the start and incrementing it for each square we move, and their coordinates in the path. 

We note that for a pair of starting skip square $(x_1, y_1)$ with index $i_1$ and the ending skip square $(x_2, y_2)$ with index $i_2$, we can find the manhattan distance between them, $d = |x_1 - x_2| + |y_1 - y_2|$.

We can determine the number of squares skipped as $|i_1 - i_2| - d$. We note that based on the problem statement, we want $d \leq 2$ and $|i_1 - i_2| - d \geq 100$.

For each path on the grid, we can therefore search in a diamond shape around it for valid positions with manhattan distances less than or equal to 2, and check if $|i_1 - i_2| - d \geq 100$. We then return the number of valid pairs.

### Code Complexity

**Time Complexity:** $O(N \times M + P)$

* $N$ is the number of rows in the grid.
* $M$ is the number of columns in the grid.
* $P$ is the length of the path.

The initial BFS has a time complexity of $O(N\times M)$. Afterwards, for each position on the path, we will do an $O(1)$ check on all the valid squares in its radius, followed by an $O(1)$ check to determine if the number of squares skipped is greater than or equal to 100.

**Additional Space Complexity:** $O(N \times M)$

The initial BFS needs to store a visited vector, and this dominates the size of the path vector.

**Final answer:** 1426.

## Part B

### Abridged Problem Statement

You are given a grid where 'S' is the start position, 'E' is the ending position, and '.' is empty space while '#' are walls. It is known that there is only one valid path from the start to the end. 

You can skip walls from an empty space **once** by moving at most **twenty** squares such that the ending square is an empty space. Let $x$ be the number of moves saved by performing the skip. Determine how many pairs of starting and ending skip squares there are such that the number of moves saved by performing the skip is more than or equal to 100.

### Solution

First, we acknowledge that the above solution works, but due to the high constant factor of checking all squares in a 20 manhattan distance radius, it is actually slower than the solution proposed below.

We similarly note from part A that for a pair of starting skip square $(x_1, y_1)$ with index $i_1$ and the ending skip square $(x_2, y_2)$ with index $i_2$, we can find the manhattan distance between them, $d = |x_1 - x_2| + |y_1 - y_2|$. We can determine the number of squares skipped as $|i_1 - i_2| - d$. We note that based on the problem statement, we want $d \leq 20$ and $|i_1 - i_2| - d \geq 100$.

Due to the high constant factor of checking all squares in a 20 manhattan distance radius, we can have an alternative solution. We simply check all pairs of starting skip square $(x_1, y_1)$ and the ending skip square $(x_2, y_2)$ on the path, and check if both conditions above are satisfied. This, in practice, gives a lower runtime than the solution in part A for large distances.

We can optimize this further by noting that the first 100 squares from the current square will never yield a skip distance less than 100, so we can skip it. Furthermore, if a current pair of skip squares has a manhattan distance that is more than 20, we also note that the next skip distance - 20 squares will also never be valid. So, we can skip these set of squares. In practice, this gives about a 10 times constant time factor speedup.

### Code Complexity

**Time Complexity:** $O(N \times M + P^2)$

* $N$ is the number of rows in the grid.
* $M$ is the number of columns in the grid.
* $P$ is the length of the path.

The initial BFS has a time complexity of $O(N\times M)$. Afterwards, for every pair of positions on the path, we will do an $O(1)$ check to determine if the number of squares skipped is greater than or equal to 100, and an $O(1)$ check to determine if the manhattan distance between the two positions is less than or equal to 20.

**Additional Space Complexity:** $O(N \times M)$

The initial BFS needs to store a visited vector, and this dominates the size of the path vector.

**Final answer:** 1000697.
