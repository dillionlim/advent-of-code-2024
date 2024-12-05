# Day 1: Historian Hysteria

## Input Bounds

Based on preliminary data analysis, all the input can be fit into an unsigned 32-bit integer. In fact, all numbers seem to be unsigned 5-digit numbers.

## Part A

### Abridged Problem Statement

You are given an input of $n$ rows. Each row has 2 numbers, $a$ and $b$. For each column, sort the numbers, and find the sum of the absolute difference between the numbers in each row once sorted.

### Solution

Simply parse the input, and sort the columns. Then, iterate through each row and find the total sum of the absolute difference of the numbers in the same row.

### Code Complexity

**Time Complexity:** $O(N \log N)$

* $N$ is the number of rows.

Each column needs to be sorted.

**Additional Space Complexity:** $O(N)$

Each column is copied out to a temporary vector before being sorted. 

**Final answer:** 1666427.

## Part B

### Abridged Problem Statement

You are given an input of $n$ rows. Each row has 2 numbers, $a_i$ and $b_i$. Let $A = \{a_0,a_1,\cdots,a_n\}$ and $B = \{b_0,b_1,\cdots,b_n\}$.

Find 
$$\sum_{a\in A} a \cdot \texttt{cnt($a$)},$$
where $\texttt{cnt(\(x\))}$ is the number of times $x$ appears in $B$.

### Solution

Use a hashmap on $B$ to get the counts of each number. Then, iterate through $A$ and calculate the total sum based on the above formula accordingly.

### Code Complexity

**Time Complexity:** $O(N)$

* $N$ is the number of rows.

Hashmaps have an average $O(1)$ insertion and read. Each number is iterated once.

**Additional Space Complexity:** $O(N)$

At worst, the hashmap can take up to $O(N)$ space to store all the values in $B$.

**Final answer:** 24316233.