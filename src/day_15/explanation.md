# Day 15: Warehouse Woes

## Input Bounds

From preliminary data analysis, all the characters in the grid are either '#', '.', '@' or 'O'.

All the characters for the movement are either '>', '^', '<' or 'v'.

## Part A

### Abridged Problem Statement

You are given a grid, where `#` is a wall, `.` is empty space, `@` is your starting position and `O` are boxes.

For each step, if you come into contact with a box, you push the box if the box will not collide with a wall. You can push chains of boxes. If you cannot move the box or will walk into a wall, you do not move.

After executing all movements, for each box location, sum up the value of $100x + y$, where $(x,y)$ is the coordinates of the box (row first).

### Solution

Simulate the movement accordingly. If you need to push a box, iterate until you reach the end of the chain of boxes, or until you reach a wall. If it is an empty space at the end of the boxes, move the starting box to that ending location.

At the end, simply sum up the required values.

### Code Complexity

**Time Complexity:** $O(K \times \max(N, M))$

* $N$ is the number of rows in the grid.
* $M$ is the number of columns in the grid.
* $K$ is the total number of movements.

For each movement, you need to move at most across the maximum of the number of rows or columns of the grid in the worst case scenario.

**Additional Space Complexity:** $O(N \times M)$

The grid is stored and is modified to track the movements.

**Final answer:** 1421727.

## Part B

### Abridged Problem Statement

You are given a grid, where `#` represents a wall of width 2, `.` is empty space of width, `@` is your starting position with an empty space to your right, and `O` are boxes of width 2.

For each step, if you come into contact with a box, you push the box if the box will not collide with a wall. You can push chains of boxes, including those that partially overlap. If you cannot move the box or will walk into a wall, you do not move.

After executing all movements, for each box location, sum up the value of $100x + y$, where $(x,y)$ is the coordinates of the left edge of the box (row first).

### Solution

First, we construct the expanded grid for ease of movement. In particular, the boxes will be represented as `[]`. At every step, we are either moving left/right or up/down. This involves two separate handling of cases:

* **Left/right:** In this case, we simply iterate to find the end of the chain of boxes. We then need to iterate to replace all the boxes with their new locations, since `.[]` will end up as `[].`.
* **Up/down:** This will be a two step process.

    We first perform a check using BFS/DFS to see if the boxes can be pushed. There are two cases here:

    * Those of the form:
    ```
    []
    []
    ```

    For these, we simply need to run a BFS/DFS on the box directly above.

    * Those of the form:
    ```
    [][]
     []
    ```

    For these, either of the top boxes may be missing, so we need to verify their presence independently. If the box is present, we run a BFS/DFS on that box.

    For each box, we also need to verify that the two spaces above it are not walls (`#`).

    If all boxes are possible to be pushed, we will then recursively push the boxes. We handle both cases separately:

    * For those of the form:
    ```
    []
    []
    ```

    We will recursively push the box directly above, before we update the box below.

    * Those of the form:
    ```
    [][]
     []
    ```

    We will check which of the two boxes above exists. We will then recursively push those boxes, before we update the box below.

    * Those without boxes above, or after recursively updating all other boxes that come after it.

    We will then move the box one row in the direction of movement, and set the original position to empty space.

At the end, we simply run a check to look for `[` and sum up the required coordinates.

### Code Complexity

**Time Complexity:** $O(K \times X)$

* $K$ is the total number of movements.
* $X$ is the average number of boxes pushed.

For each movement, we need to determine whether the boxes can be pushed (for vertical movements), and subsequently update the box positions. This requires $O(X)$ time for each movement.

**Additional Space Complexity:** $O(N \times M)$

* $N$ is the number of rows in the grid.
* $M$ is the number of columns in the grid.

The grid is stored and modified.

**Final answer:** 1463160.
