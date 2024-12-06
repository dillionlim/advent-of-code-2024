# Day 4: Ceres Search

## Input Bounds

From preliminary data analysis, the input is multiple lines of characters, containing 'X', 'M', 'A' and 'S'.

## Part A

### Abridged Problem Statement

Given an input of multiple lines of characters, find the number of occurrences of the string 'XMAS', which can appear in any of the 8 directions.

### Solution

For each occurrence of 'X', perform a search in all 8 directions for the count of the number of 'XMAS'.

### Code Complexity

**Time Complexity:** $O(N \times M)$

* $N$ is the number of rows in the grid,
* $M$ is the number of columns in the grid.

Each cell is iterated through once, and the maximum number of searches from each cell is constant ($8 \times 4 = 32$, for 8 directions for each string of length 4).

**Additional Space Complexity:** $O(N \times M)$

The grid is stored in a `Vec<Vec<char>>`.

**Final answer:** 2575.

## Part B

### Abridged Problem Statement

Given an input of multiple lines of characters, find the number of occurrences of the multiline string,
```
M.S
.A.
M.S
``` 
which can appear in any orientations, and `.` can be any character.

### Solution

For each occurrence of 'A', perform a search for all possible orientations for the count of the multiline string X-MAS.

### Code Complexity

**Time Complexity:** $O(N \times M)$

* $N$ is the number of rows in the grid,
* $M$ is the number of columns in the grid.

Each cell is iterated through once, and the maximum number of searches from each 'A' cell is constant ($4 \times 4 = 16$, for 4 possible orientation of the X-MAS strings for each orientation of length 4).

**Additional Space Complexity:** $O(N \times M)$

The grid is stored in a `Vec<Vec<char>>`.

**Final answer:** 2041.
