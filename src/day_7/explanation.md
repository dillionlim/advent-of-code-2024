# Day 7: Bridge Repair

## Input Bounds

From preliminary data analysis, the target number is an unsigned 64-bit integer, and the rest of the numbers are unsigned 32-bit integers. More importantly, 0 does not exist as a valid number.

## Part A

### Abridged Problem Statement

There are $Q$ queries.

For each query, you have a target number, $N$, and a list of numbers $\{a_0, a_1, \cdots\}$. A list is *satisfactory* if there exists a way to place $+$ and $\times$ operators between the numbers such that, when evaluated left-to-right, it gives the result $N$.

Find the total sum of all target numbers of all the *satisfactory* queries.

### Solution (Old)

We wil run a brute force solution, with pruning. Since 0 does not exist, we know that any addition or multiplication operation will only increase the number. Therefore, if we exceed the target, we can stop going down that branch.

### Solution (Optimized)

**Observation.** If we work backwards, the target number can only be formed from a multiplication with the last number in our array if it is divisible by the last number in our array. 

That is, suppose that there exists a chain of numbers $t_0, t_1, t_2, \cdots$ such that $t_{i+1}$ can be formed from $t_i$ by some operation (addition or multiplication), and $t_0$ is our target number. Then, 
* $t_{i+1}$ can only be formed from a multiplication if $t_i | t_{i+1}$.
* $t_{i+1}$ can only be formed from an addition if $t_i > t_{i+1}$, since negative numbers and zero do not exist in our input.

We can use this observation to prune a lot of unnecessary branches in our code.

Therefore, we iterate from the back of the array of numbers. 

* We check if, for each number $a_i$, if there exists $t_i$ such that $t_i \cdot a_i = t_{i+1}$. This boils down to checking if the $t_{i+1}$, the next number, is divisible by $a_i$, the current number that we are looking at. If it is divisible, then $t_i$ is simply the quotient.
* Similarly, we check if, for each number $a_i$, if there exists $t_i$ such that $t_i + a_i = t_{i+1}$. This boils down to checking if the $t_{i+1} > a_i$. If it is divisible, then $t_i = t_{i+1} - a_i$.

*This optimization brings the runtime down from 63 ms to 17 ms, about a 3 to 4 times speedup!*

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

### Solution (Old)

We wil run a brute force solution, with pruning. Since 0 does not exist, we know that any addition, multiplication or concatenation operation will only increase the number. Therefore, if we exceed the target, we can stop going down that branch.

In particular, to avoid overflow for numbers, we will count the number of digits and prematurely terminate it if the concatenation of the two input numbers will have more digits than the target number.

### Solution (Optimized)

We use the same observation as in Part A. The bolded statements are the new statements for the concatenation operation.

That is, suppose that there exists a chain of numbers $t_0, t_1, t_2, \cdots$ such that $t_{i+1}$ can be formed from $t_i$ by some operation (addition, multiplication or concatenation), and $t_0$ is our target number. Then, 
* $t_{i+1}$ can only be formed from a multiplication if $t_i | t_{i+1}$.
* $t_{i+1}$ can only be formed from an addition if $t_i > t_{i+1}$, since negative numbers and zero do not exist in our input.
* **$\boldsymbol{t_{i+1}}$ can only be formed from a concatenation if $\boldsymbol{t_{i+1}}$ starts with $\boldsymbol{t_i}$.**

We can use this observation to prune a lot of unnecessary branches in our code.

Therefore, we iterate from the back of the array of numbers. 

As with before
* We check if, for each number $a_i$, if there exists $t_i$ such that $t_i \cdot a_i = t_{i+1}$. This boils down to checking if the $t_{i+1}$, the next number, is divisible by $a_i$, the current number that we are looking at. If it is divisible, then $t_i$ is simply the quotient.
* Similarly, we check if, for each number $a_i$, if there exists $t_i$ such that $t_i + a_i = t_{i+1}$. This boils down to checking if the $t_{i+1} > a_i$. If it is divisible, then $t_i = t_{i+1} - a_i$.
* **We check if, for each number $\mathbf{a_i}$, if there exists $\mathbf{t_i}$ such that $\mathbf{t_i}t_i$ is a prefix of $\mathbf{t_{i+1}}$. This boils down to checking if $\mathbf{a_i}$ is a suffix of $\mathbf{t_{i+1}}$.**

To do this operation efficiently, we will simply determine if $a_i$ is a suffix of $t_{i+1}$ by counting the number of digits in $a_i$, using logarithms. Suppose that $a_i$ has $d$ digits. We can then mask the last $d$ digits of $t_{i+1}$ using $t_{i+1} \bmod{10^d}$. If $t_{i+1} \bmod{10^d} = a_i$, then $a_i$ is a suffix of $t_{i+1}$. We can then find $t_i = \frac{t_{i+1}-a_i}{10^d}$.

*This optimization brings the runtime down from over 1 second to 20 ms, over a 50 times speedup!*

### Code Complexity

**Time Complexity:** $O(Q \times 3^N)$

* $Q$ is the number of queries.
* $N$ is the average length of the list for each query.

In the worst case, we will need to explore all combinations of operators for each query.

**Additional Space Complexity:** $O(3^N)$

In the worst case, we will need to store the result of the combinations of operators for each query at every level, and the queue will grow to a maximum length of $3^N$.

**Final answer:** 500335179214836.