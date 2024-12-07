# Day 7: Bridge Repair

## Input Bounds

From preliminary data analysis, the target number is an unsigned 64-bit integer, and the rest of the numbers are unsigned 32-bit integers. More importantly, 0 does not exist as a valid number.

## Part A

### Abridged Problem Statement

There are $Q$ queries.

For each query, you have a target number, $N$, and a list of numbers $\{a_0, a_1, \cdots\}$. A list is *satisfactory* if there exists a way to place $+$ and $\times$ operators between the numbers such that, when evaluated left-to-right, it gives the result $N$.

Find the total sum of all target numbers of all the *satisfactory* queries.

### Solution

We wil run a brute force solution, with pruning. Since 0 does not exist, we know that any addition or multiplication operation will only increase the number. Therefore, if we exceed the target, we can stop going down that branch.

### Code Complexity

**Time Complexity:** $O(Q \times 2^N)$

* $Q$ is the number of queries.
* $N$ is the average length of the list for each query.

In the worst case, we will need to explore all combinations of operators for each query.

**Additional Space Complexity:** $O(2^N)$

In the worst case, we will need to store the result of the combinations of operators for each query at every level, and the queue will grow to a maximum length of $2^N$.

**Final answer:** 10741443549536.

## Part B

### Abridged Problem Statement

There are $Q$ queries.

For each query, you have a target number, $N$, and a list of numbers $\{a_0, a_1, \cdots\}$. A list is *satisfactory* if there exists a way to place $+$, $\times$ or $||$ operators between the numbers such that, when evaluated left-to-right, it gives the result $N$.

The $||$ operator is the *concatenation* operator, which concatenates two numbers.

Find the total sum of all target numbers of all the *satisfactory* queries.

### Solution

We wil run a brute force solution, with pruning. Since 0 does not exist, we know that any addition, multiplication or concatenation operation will only increase the number. Therefore, if we exceed the target, we can stop going down that branch.

In particular, to avoid overflow for numbers, we will count the number of digits and prematurely terminate it if the concatenation of the two input numbers will have more digits than the target number.

### Code Complexity

**Time Complexity:** $O(Q \times 3^N)$

* $Q$ is the number of queries.
* $N$ is the average length of the list for each query.

In the worst case, we will need to explore all combinations of operators for each query.

**Additional Space Complexity:** $O(2^N)$

In the worst case, we will need to store the result of the combinations of operators for each query at every level, and the queue will grow to a maximum length of $3^N$.

**Final answer:** 500335179214836.