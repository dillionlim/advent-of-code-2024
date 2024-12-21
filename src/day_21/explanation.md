# Day 21: Keypad Conundrum

## Input Bounds

Based on preliminary data analysis, there are only 5 lines, each of which have 3 numeric characters followed by 'A'.

## Part A and B

Part A and B have the same solution for today's problem, so it will be simplified accordingly.

### Abridged Problem Statement

You have a number keypad of the form
```
789
456
123
 0A
```

and a direction keypads of the form
```
 ^A
<v>
```

In both keypads above, the A button presses the button which the pointer is pointing to.

Initially, the pointer for the numbered keypad starts at the A button. You need to use the directional keypad to key in the code. This generates a series of movements on this keypad. Let this series of movement be $M_1$. 

There is another pointer for the directional keypard, starting at the A button. You need to use another directional keypad to key in the code. This generates another series of movements on this second directional keypad. Let this series of movements be $M_2$.

In general, we can recursively generate a series of movements $M_n, n\geq 1$. Let the shortest possible number of movement of $M_n$ be $|M_n|$. 

For each code, the first three digits form a number $C_i$. The *complexity* of the code is $C_i \times |M_n|_i$. Find the sum of the complexity for all codes.

For part A, $n=2$. For part B, $n=25$.

### Solution

First, we find the initial shortest set of movements required to input the code using the numbered keypad. We can do this by performing a BFS. 

We note that the shortest path to key in the grid is the shortest path to key in each individual pair of buttons. We can prove this trivially by contradiction since if there exists a shorter path between two buttons, we can replace the path between the two buttons in our current shortest path with this shorter path, which will reduce the path length.

Therefore, for each pair of buttons, we simply store a `parents` array, which stores the predecessors and direction of the previous cell of the current cell. 

After our BFS pops the ending button, we can backtrack to find all shortest paths in the grid. We now have a vector of all shortest paths in the grid between two buttons.

We will then need to find the minimum $|M_n|$ for these pair of buttons. We note that we can recursively generate a set of directions to input $M_{i+1}$ from $M_i$, with a base case of $M_1$, our initial set of directions. 

Therefore, we let $f(i, j, r)$ be the minimum number of movements in the $r^{\text{th}}$ layer of iteration that is needed to move from button $i$ to $j$. To calculate $f(i, j, r)$, we will try all valid shortest paths from $i$ to $j$. For each such valid shortest path, $p_0, p_1, p_2, \cdots, p_n$, where $i = p_0, j=p_n$, the number of movements required to generate the path is $\sum_{i=0}^{n-1} f(p_i, p_{i+1}, r-1)$. We therefore can recursively express the solution to $f(i, j, r)$.

Let $P = \{P_0, P_1, \cdots\}$ be the set of shortest paths from $i$ to $j$. For each $P_x$, let $p_{x_0}, p_{x_1}, p_{x_2}, \cdots, p_{x_n}$, where $i = p_{x_0}, j=p_{x_n}$.

Then, we can express:

$$f(i, j, r) = \min_{P_i \in P} \left(\sum_{j=0}^{n-1} f(p_{i_j}, p_{i_{j+1}}, r-1) \right)$$

To find the valid shortest paths, we can once again use BFS with backtracking as elaborated upon earlier.

We note that a naive recurrence solution will take an exponential time with respect to the number of robots. Therefore, we can use DP as a speedup.

We will hash the starting position, `s`, ending position, `e`, and the number of robots, `r`, as a 32-bit unsigned integer, noting that all 3 values fit in a unsigned 8-bit integer. Therefore, we can hash it as `s | (e << 8) | (r << 16)`. We can then store it in a DP hashmap for memoization.

### Code Complexity

**Time Complexity:** $O(N \times M \times P \times R)$

* $N$ is the number of codes.
* $M$ is the length of each code.
* $P$ is the average number of shortest numbered keypad paths for each code.
* $R$ is the number of robots.

For each code, we iterate through each pair of characters. This takes $O(N \times M)$ time.

For each pair of characters, we perform a BFS to get the initial set of directions, $M_0$, which takes $O(V + E)$ time, where $V = 10$ and the maximum $E = 4$ based on our adjacency list, which is effectively a constant value. 

Then, for each shortest path obtained, we perform the DP to find the minimum $|M_R|$. Since the state space of the DP is $R \times D \times D$, where $D$ is the number of vertices in the directional keypad, and $D$ is a constant value (5), the time complexity for the computation for each shortest path is $O(R)$.

Thus, the overall time complexity of the solution is $O(N \times M \times P \times R)$.

**Additional Space Complexity:** $O(N \times M + R)$

Storing the input takes $O(N\times M)$ space. The maximum size of the queue in each BFS is $O(V)$, which is effectively constant. The maximum size of the DP map is the state space, which is $O(R)$ as explained above.

**Final answer (Part A):** 162740.

**Final answer (Part B):** 203640915832208.