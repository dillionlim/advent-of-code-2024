# Day 22: Monkey Market

## Input Bounds

From preliminary data analysis, all the values are integers that fit in an unsigned 64-bit integer.

## Part A

### Abridged Problem Statement

You are given a list of initial secret codes, which is an integer.

For each integer, let this be $i_0$. Then, we can generate $i_n$ from $i_{n-1}$ by doing the following:

* First, calculate the result of multiplying $i_{n-1}$ by 64. Then, find the XOR of the original number and this result. Then, find the result modulo 16777216. Let this result be $a$.
* Next, calculate the result of the floor of dividing the previous result, $a$, by 32. Then, find the XOR of the $a$ and this result. Then, find the result modulo 16777216. Let this result be $b$.
* Next, calculate the result of multiplying the previous result, $a$, by 2048. Then, find the XOR of the $b$ and this result. Then, find the result modulo 16777216. This result is $i_n$.

Find the value of $i_{1999}$. Sum up this value across all initial starting codes.

### Solution

Simulate the described process above. However, we realise that since all numbers above are powers of 2, we can use bitwise operations to represent them.

* Multiplying by $2^x$ can be done by `a << x`.
* Dividing by $2^x$ can be done by `a >> x`.
* Finding $a \bmod 2^x$ can be done by `a &` $(2^x -1)$.

### Code Complexity

**Time Complexity:** $O(N \times M)$

* $N$ is the total number of initial codes.
* $M$ is the total number of iterations for each code.

For each code, we need to iterate $M$ times. Each iteration takes $O(1)$ time since it is 9 bitwise operations.

**Additional Space Complexity:** $O(N)$

The initial code needs to be stored in a vector.

**Final answer:** 17005483322.

## Part B

### Abridged Problem Statement

You are given a list of initial secret codes, which is an integer.

For each integer, let this be $i_0$. Then, we can generate $i_n$ from $i_{n-1}$ by doing the following:

* First, calculate the result of multiplying $i_{n-1}$ by 64. Then, find the XOR of the original number and this result. Then, find the result modulo 16777216. Let this result be $a$.
* Next, calculate the result of the floor of dividing the previous result, $a$, by 32. Then, find the XOR of the $a$ and this result. Then, find the result modulo 16777216. Let this result be $b$.
* Next, calculate the result of multiplying the previous result, $a$, by 2048. Then, find the XOR of the $b$ and this result. Then, find the result modulo 16777216. This result is $i_n$.

Given the series of values $\{i_0, i_1, i_2, \cdots, i_{1999}\}$, create a similar set of values $P = \{j_0, j_1, \cdots j_{1999}\}, j_n = i_n\bmod 10$.

Create a similar set of values for each of the initial codes.

Each value for $j_n$, for $n\geq 4$ is defined by the last 4 pairwise differences. That is, $j_n$ is associated with the chain $(j_{n-3} - j_{n-4}, j_{n-2} - j_{n-3}, j_{n-1} - j_{n-2},j_n - j_{n-1})$.

For each possible chain, find the maximum sum of $j_n$ across all initial starting values given that chain.

### Solution

We create a sliding window of size 5 to find the next chain given the current chain. We will then map each chain to the last element. For each chain, we will then sum up the associated element for each starting value.

As an optimization, we note that since the differences lie in the range $[-9, 9]$, we can hash the values as a base 19 integer.

For a chain $(a, b, c, d)$, we can express it as a base 19 integer by taking $[(a + 10) \times 19^3 + (b + 10) \times 19^2 + (c + 10) \times 19^1 + (d + 10) \times 19^0] \bmod{19^4}$.

We note that since $10 \times 19^3 + 10 \times 19^2 + 10 \times 19^1 + 10\times 19^0 = 65160$, the equation above simplifies to $(a \times 19^3 + b \times 19^2 + c \times 19^1 + d \times 19^0 + 65160) \bmod{19^4}$.

We can then hash this value instead.

### Code Complexity

**Time Complexity:** $O(N \times M)$

* $N$ is the total number of initial codes.
* $M$ is the total number of iterations for each code.

For each code, we need to iterate $M$ times. Each iteration takes $O(1)$ time since it is 9 bitwise operations, followed by the sliding window to generate the hashed value which also takes $O(1)$ time.

The final iteration to find the maximum value can be found in $O(N)$.

**Additional Space Complexity:** $O(N)$

The initial code needs to be stored in a vector. The sliding window is of a constant size of 5.

**Final answer:** 1910.
