# Day 6: Guard Gallivant

## Input Bounds

From preliminary data analysis, the guard will start in the `^` position. All other characters will be `.` or `#`. The movement of the guard is guaranteed to terminate for part A.

## Part A

### Abridged Problem Statement

Given a grid, where `.` represents empty space and `#` represents an obstacle, and `^` represents the initial position of the guard, find the total number of unique cells that the guard will cross before it exits the boundary of the grid. The guard will turn clockwise on encountering an obstacle.

### Solution

Find the initial coordinates of the guard, and simulate the movement of the guard. Record cells that the guards has already crossed to avoid double counting.

### Code Complexity

**Time Complexity:** $O(M \times N)$

* $M$ is the number of rows in the grid.
* $N$ is the number of columns in the grid.

We iterate through each cell to simulate the movement of the guard.

**Additional Space Complexity:** $O(M \times N)$

The grid is also used for storing the visited cells of the guard.

**Final answer:**

## Part B

### Abridged Problem Statement

Given a grid, where `.` represents empty space and `#` represents an obstacle, and `^` represents the initial position of the guard, find the total number of unique cells that the guard will cross before it exits the boundary of the grid. The guard will turn clockwise on encountering an obstacle.

Find the number of cells where placing a `#` will cause the guard to walk in a loop.

### Solution

First, we observe that placing obstacles on any cell that is not on the original guard's path (in part A) will not lead to a loop.

So, we only need to test whether placing obstacles on a given cell on the original guard's path will lead to a loop. For this, we can use a hash set to maintain the set of `(x, y, direction)` at every turn, and there exists a loop if we encounter the same set of `(x, y, direction)` again.

As an improvement, one can hash the coordinates into a single integer, and use a bitmask to represent the direction of the current represented. That is, the value of the `direction_index`-th bit will represent whether that direction has been visited. We can then reuse the vector that is functioning as our hash set to avoid reconstruction costs.

### Code Complexity

**Time Complexity:** $O(M^2 \times N^2)$

* $M$ is the number of rows in the grid.
* $N$ is the number of columns in the grid.

We iterate through each cell to simulate the movement of the guard. For each cell on the path of the guard, in the worst case, we need to iterate through the grid again to determine if it causes a loop.

**Additional Space Complexity:** $O(M \times N)$

The hashmap can store up to all the cells, in the worst case scenario. 

**Final answer:** 1723.
