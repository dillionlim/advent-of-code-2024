# Day 2: Red-Nosed Reports

## Input Bounds

From preliminary data analysis, we know that the numbers are all unsigned 32-bit integers.

We also know that each row must have at least 2 integers, but we will not use that fact here (we will still handle it).

## Part A

### Abridged Problem Statement

You have $Q$ queries. 

Each query consists of a list of $N$ numbers. Each query is *safe* if it is either strictly decreasing or strictly increasing, and adjacent numbers differ by a value between 1 and 3 (inclusive). 

Count the number of *safe* queries.

### Solution

* If the row only has 1 number, it is safe trivially.
* Otherwise, we will establish whether the row is increasing or decreasing by examining the first 2 numbers of the row. 
* Based on the established direction, we will then examine the rest of the numbers to determine if they fall within the acceptable range (1 to 3).
* This can be implemented quite easily in Rust with a sliding window (`array.window(2)`).

### Code Complexity

**Time Complexity:** $O(N \times M)$

* N is the number of reports
* M is the number of levels

Trivially, each number is looked at only once.

**Additional Space Complexity:** $O(1)$

A constant amount of extra space is used.

**Final answer:** 321.

## Part B

### Abridged Problem Statement

You have $Q$ queries. 

Each query consists of a list of $N$ numbers. Each query is *safe* if it is either strictly decreasing or strictly increasing, and adjacent numbers differ by a value between 1 and 3 (inclusive), or if it has at most 1 number that can be removed to make the query safe.

Count the number of *safe* queries.

### Solution

Firstly, note that anything less than 3 numbers will be trivially safe:
* 1 number is trivially safe.
* If the difference of the 2 numbers fall outside the range, then we can simply remove one of the numbers.

For 3 numbers, a check simply needs to be done to check if any of the 2 numbers will form a valid sequences. This is 3 checks, and can be handled quite elegantly with a cyclic check.

Otherwise, things will be slightly more complicated. We need to establish the **trend** of the numbers first:
* Note that our method for Day 2A will not work any more, since there is a possibility that the removal of $a_i$ will flip the direction of the sequence from that of the first 2 numbers.

For example, 8 9 8 7 6, the removal of the first number will make the sequence decreasing instead and make the sequence valid. So, this possibility must be considered as well. 
* Since we can only remove at most 1 number, looking at the first 4 numbers is sufficient to make a decision on the direction of the sequence (with the majority of the elements winning). 

Consider $a_0, a_1, a_2, a_3$. There will be 3 inequalities between them. If the inequalities are all the same, then a general trend has been set. 

Otherwise, we note that the number of inequalities will differ by at most 1 (either 2 are increasing, or 2 are decreasing). This leaves a single pair of numbers that violate the trend. We can then remove a single number to make the whole sequence of 3 numbers increasing or decreasing. 

**Therefore, we can establish the direction of the numbers by simply looking at the first 4 numbers.**

We will then consider the case of **unsafe** number pairs, which is a pair of numbers $(a_i, a_{i+1})$ that does not fulfil either the trend or bounds.

For a pair of numbers $(a_i, a_{i+1})$ that fails either condition above, we can simply test if the removal of the number will resolve it.
* Removing $a_{i+1}$ means that we would be determining if $(a_i, a_{i+2})$ violate any conditions. However, if $a_{i+1}$ is the last number of the row, then it can simply be removed.
* Removing $a_i$ means that we would be determining if $(a_{i-1},a_{i+1})$ violate any conditions. However, if $a_i$ is the first number of the row, then it can also simply be removed.
* If the removal of both numbers is still unable to resolve the violation, then it is impossible to resolve the violation for the row.

We note that there can only be a maximum of one violation per row, so we need to track whenever a violation has been recorded, so the next violation will automatically result in the row being *unsafe*.

### Code Complexity

**Time Complexity:** $O(N \times M)$

* N is the number of reports
* M is the number of levels

Each report is only iterated through once. Each level is only iterated through once, since we do a constant number of checks for each level. 

**Additional Space Complexity:** $O(1)$

Only a constant amount of extra space is used.

**Final answer:** 386.