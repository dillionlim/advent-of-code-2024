# Day 8: Resonant Collinearity

## Input Bounds

Based on preliminary data analysis, the grid contains either '.' or alphanumeric characters.

## Part A

### Abridged Problem Statement

Given an input grid containing '.' and alphanumeric characters, find how many grid spaces, $G$, exist such that there exists 2 points, $A$ and $B$ such that $\text{dist}(A,G) = 2 \times \text{dist}(B,G)$ and the character on $A$ is the same as that on $B$.

### Solution

Use a hashmap to store the coordinates of all the points with the same character. 

Then, for each character type, iterate through all pairs of points and find the 2 points that satisfy the condition above. Specifically, given points $A (x_1, y_1)$ and $B (x_2, y_2)$, the two points are given by:

* Taking $A$ as the midpoint, using the midpoint theorem, 
$$(x_1, y_1) = \left(\frac{x_2+x_3}{2}, \frac{x_2+x_3}{2}\right)$$
Then, $x_3 = 2x_1 - x_2$ and $y_3 = 2y_1 - y_2$.
* Taking $B$ as the midpoint, using the midpoint theorem, 
$$(x_2, y_2) = \left(\frac{x_1+x_4}{2}, \frac{x_1+x_4}{2}\right)$$
Then, $x_4 = 2x_2 - x_1$ and $y_4 = 2y_2 - y_1$.

So, the two points are given by $(2x_1-x_2, 2y_1-y_2)$ and $(2x_2-x_1, 2y_2-y_1)$.

We use a `seen` array to determine if that point is already counted to avoid double counting.

### Code Complexity

**Time Complexity:** $O(N \times M + K^2)$

* $N$ is the number of rows of the grid.
* $M$ is the number of columns of the grid.
* $K$ is the number of points in the largest group of characters.

We need to scan all the points in the grid to classify them into groups. Of these, we need to check each pair of points in each group, of which there are $\frac{K(K-1)}{2}$ pairs. We then do an $O(1)$ check for each pair.

**Additional Space Complexity:** $O(N \times M)$

In the worst case, all points need to be stored in the hash map.

**Final answer:** 332.

## Part B

### Abridged Problem Statement

Given an input grid containing '.' and alphanumeric characters, find how many grid spaces, $G$, exist such that it is collinear with 2 points, $A$ and $B$ such that the character on $A$ is the same as that on $B$.

### Solution

We start off by using the hashmap to classify the points by their characters, as with part A.

Then, we effectively want to find the number of collinear lattice points given any 2 points. We can view this by determining the "unit" vector between the 2 points, by finding the GCD between the two points. This gives us the smallest step in either direction possible between both points. Then, we simply find any linear multiple of this "unit" vector, added to any point of our choice to obtain all the lattice points that are collinear with the two points.

That is, for any two points $\begin{bmatrix}x_1\\y_1\\\end{bmatrix}$ and $\begin{bmatrix}x_2\\y_2\\\end{bmatrix}$, we know the direction vector, $d$ is given by
$$\vec{d} = \begin{bmatrix}
x_1-x_2 \\
y_1-y_2 \\
\end{bmatrix}$$

Then, we define the "unit" vector $\hat{d}$ as
$$\hat{d} = \begin{bmatrix}
\frac{x_1-x_2}{\gcd(x_1,x_2)} \\
\frac{y_1-y_2}{\gcd(y_1,y_2)} \\
\end{bmatrix}$$

Then, the set of lattice points collinear with both points is given by 
$\begin{bmatrix}x_1\\y_1\\\end{bmatrix} + \lambda \hat{d}, \lambda \in \mathbb{Z}$, and we can simply filter by the points that are within the grid.

We also use a `seen` array to avoid revisiting or recounting points.

### Code Complexity

**Time Complexity:** $O(N \times M + K^2 \cdot \max(N, M))$

* $N$ is the number of rows of the grid.
* $M$ is the number of columns of the grid.
* $K$ is the number of points in the largest group of characters.

We need to scan all the points in the grid to classify them into groups. Of these, we need to check each pair of points in each group, of which there are $\frac{K(K-1)}{2}$ pairs. Then, we need to find all collinear points for each pair of points, which, in the worst case, will take $O(\max(N,M))$ time.

**Additional Space Complexity:** $O(M \times N)$

In the worst case, all points need to be stored in the hash map.

**Final answer:** 1174.
