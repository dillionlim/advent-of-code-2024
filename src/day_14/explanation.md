# Day 14: Restroom Redoubt

## Input Bounds

From preliminary data analysis, all coordinates and velocities are signed 32-bit integers.

## Part A

### Abridged Problem Statement

Given an initial starting point $(x, y)$ and a constant velocity vector $(v_x, v_y)$, find the position of the point after $t = 100$ seconds, given that if the point goes out of bounds, it wraps around. That is, if $x$ goes out of bounds, the value is $x \bmod x_{max}$, where $x_{max}$ is the length of the $x$ direction and similar for $y$.

Classify the points into 4 quadrants, and all points on the center column or row (for odd sizes) will be ignored.

Multiply the total number of points in each quadrant to obtain the answer.

### Solution

The $(x_t, y_t)$ coordinates at time $t$ for an initial position $(x_0, y_0)$ with velocity $(v_x, v_y)$ is given by $(x_0 + v_xt, y_0 + v_yt)$. However, since they wrap around if they go out of bounds, then for a grid of size $(x_{max}, y_{max})$, the final coordinate is given by $((x_0 + v_xt) \bmod x_{max}, (y_0 + v_yt) \bmod y_{max})$.

We will need to handle the case where the final coordinates are negative, since Rust (and many other languages) returns a modulo value with the same sign as the left operand.

This is easily handled by the fact that $x \equiv x+a \pmod{a}$ for $x<0$. So, we can simply add the modulus, and take the modulo value again.

Lastly, we need to classify it into each of the quadrants. 

A point is in:
* The left half if it has $x < \lfloor \frac{x_{max}}{2} \rfloor$.
* The right half if it has $x \geq \lfloor \frac{x_{max}}{2} \rfloor + x_{max} \bmod 2$.
* The top half if it has $y < \lfloor \frac{y_{max}}{2} \rfloor$.
* The bottom half if it has $y \geq \lfloor \frac{y_{max}}{2} \rfloor + y_{max} \bmod 2$.
* Otherwise, it belongs to the middle row or column, and should be discarded.

By determining whether a point belongs to the left/right half, and the top/bottom half, we can assign it into a quadrant.

We then multiply the total number of points in each quadrant to arrive at our answer.

### Code Complexity

**Time Complexity:** $O(N)$

* $N$ is the number of lines of input.

For each line of input, the final position and its quadrant is computed in $O(1)$ time.

**Additional Space Complexity:** $O(N)$

The inputs will be stored in a vector.

**Final answer:** 224438715.

## Part B

### Abridged Problem Statement

The robots are said to eventually form a picture of a Christmas tree. Find the minimum length of time for the robots to display the picture of the Christmas tree.

### Solution

First, we notice that the position of the robots is periodic. They will repeat every $101 \times 103$ seconds. Therefore, we only need to check the seconds from $0$ to $101 \times 103 - 1$.

Next, we need to determine a heuristic to find the "Christmas tree". Since we are not sure what the Christmas tree looks like, we need to make a guess. The Christmas tree should have the robots clustered very tightly together, so we can simply find the standard deviation of the position of all points at every point in time.

That is, for a list of coordinates of robots in the form $(x_i, y_i)$, for $i = 0, 1, \cdots, n$, we can calculate

$$ (\sigma_x, \sigma_y) = \left(\sqrt{\frac{\sum_{i=0}^n (x_i - \overline{x})^2}{n-1}}, \sqrt{\frac{\sum_{i=0}^n (y_i - \overline{y})^2}{n-1}} \right),$$

where 
* $\overline{x}$ and $\overline{y}$ refers to the mean of $x_i$ and $y_i$ respectively.
* $n$ is the total number of robots. 

We can then set a threshold standard deviation for both $\sigma_x$ and $\sigma_y$ to manually filter through to look for the Christmas tree.

We can then conduct a binary search on this threshold. First, we conduct a preliminary analysis on the standard deviation of the points in the grid, and we notice that they appear to generally be about 28.0 to 30.0. 

* We then make a guess for the threshold to be 20.0, but this yields no results. 
* We then increase the threshold to 25.0, and for my input, fortunately, this yields a singular result at $t=7603$, and visual inspection shows that it is indeed the desired Christmas tree.

### Code Complexity

**Time Complexity:** $O(M \times N \times T)$

* $M$ is the number of columns of the grid.
* $N$ is the number of rows of the grid.
* $T$ is the number of robots.

For each time tick in $0$ to $M\times N-1$, we need to calculate the standard deviation of all points in the grid, which involves iterating through all the points in the grid to first obtain their mean, then calculate the standard deviation. This takes $O(T)$ time for each time tick.

Then, we need to update the positions of the points, which also involves iterating through all the points. This takes $O(T)$ time for each time tick.

So overall, the time complexity of this solution will be $O(M\times N \times T)$.

This solution takes about 500 to 900 ms to generate all solutions on my computer with a threshold of 25.0.

**Additional Space Complexity:** $O(N)$

The inputs will be stored in a vector.

**Final answer:** 7603.