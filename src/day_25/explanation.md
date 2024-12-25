# Day 25: Code Chronicle

## Input Bounds

Based on preliminary data analysis, all inputs are a 5 by 7 grid with either `#` or `.` characters.

## Part A

There is no part B today.

### Abridged Problem Statement

Given a set of 5 by 7 grids, separated by newlines, where the first line being `#####` and last line being `.....` indicates that it is a lock, and the first line being `.....` and the last line being `#####` indicates that it is a key. Each space that is `#` indicates the space being occupied.

For all pairs of keys and locks, find the number of pairs where there is no overlap between any occupied spaces.

### Solution

We represent each grid as a 35 bit number, where `0` indicates empty space and `1` indicates blank space. The grid to number order mapping does not matter, as long as it is internally consistent between keys and locks. For my implementation, I used the top-left character as the most significant bit and the bottom-left character as the least significant bit. We can classify keys and locks by simply looking at the least significant bit (LSB), where a LSB of 0 indicates a key, and a LSB of 1 indicates a lock.

Then, checking all pairs of keys and locks, we can simply determine if there is an overlap by finding the bitwise AND of the key and lock in question. We will then mask the lower 35 bits by using the bitmask $2^{36} - 1$, which is a string of 35 `1`s in binary, from the LSB. We will then check if this value is 0, which indicates no overlap. We then count the total number of values with no overlap.

### Code Complexity

**Time Complexity:** $O(K \times L)$
* $K$ is the number of keys.
* $L$ is the number of locks.

For each pair of keys and locks, we can perform a check of their overlap in $O(1)$ time.

**Additional Space Complexity:** $O(K + L)$

All keys and locks need to have their equivalent bit values stored in a vector.

**Final answer:** 3327.
