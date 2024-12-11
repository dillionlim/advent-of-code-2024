# Day 11: Plutonian Pebbles

## Input Bounds

From preliminary data analysis, it appears that all numbers are positive unsigned integers that can fit in a 32-bit unsigned integer. 

## Part A and B

Part A and B have the same solution for today's problem, so it will be simplified accordingly.

### Abridged Problem Statement

You are given a list of integers, $X$. 

For each $X_i$, perform the following step:

* If $X_i = 0$, set $X_i = 1$.
* Otherwise, if $X_i$ has an even number of digits, split $X_i$ into two numbers, with the first number being the first half of $X_i$, and the second number being the second half of $X_i$.
* Otherwise, if $X_i$ has an odd number of digits, multiply it by $2024$.

One iteration is perform the above step for all $X_i$ in $X$.

Perform the above iterations 25 times for part A, and 75 times for part $B$. Then, count the total number of numbers in $X$.

### Solution

We first note that if two numbers in $X$ have the same value $X_i$ at a certain iteration, their values will be identical for all subsequent iterations, and they will also be split into the same numbers for all subsequent iterations. 

The order of the stones therefore does not matter, and we only need to track the total number of each number $X_i$ at any iteration. We can use a hashmap to track the total number of each number $X_i$ at any iteration.

We also make a note on how to perform the splitting of numbers. The number of digits of a number $x$ is given by $d = \log_{10}x + 1$. Then, we can use a mask $m = 10^{\frac{d}{2}}$ to mask the first half and second half of $x$. The number $x$ will therefore be split into $x \bmod {m}$ and $\lfloor \frac{x}{m} \rfloor$.

### Code Complexity

**Time Complexity:** $O(k \times 2^k \times N)$

* $k$ is the number of iterations done.
* $N$ is the number of numbers in the list.

In the worst case, if all numbers are doubled every iteration, then the number of elements from a single number can grow up to $2^k$. Since there are originally $N$ numbers in the list, and there are $k$ iterations, the time complexity is therefore $O(k \times 2^k \times N)$.

However, in practice, the number of unique numbers after $n$ iterations, for any arbitrarily large $n$, is actually a constant value.

**Additional Space Complexity:** $O(2^k \times N)$

Similarly, all the unique numbers need to be stored.

**Final answer (Part A):** 175006.
**Final answer (Part B):** 207961583799296.