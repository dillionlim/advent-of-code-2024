# Day 13: Claw Contraption

## Input Bounds

From preliminary data analysis, it appears that all coordinates are positive integers.

## Part A and B

Part A and B have the same solution for today's problem, so it will be simplified accordingly.

### Abridged Problem Statement

You are given two buttons, button A that increases your coordinates by $(x_1, y_1)$ and button B that increases it by $(x_2, y_2)$. You start off at $(0,0)$. The aim is to reach $(g_x, g_y)$ by pressing each button any number of times. Let the number of times button A is pressed be $a$ and that for button B be $b$. The cost of doing so is $3a+b$. Minimise the cost to reach $(g_x, g_y)$, if possible.

You are given $n$ such inputs, and return the sum of the minimal costs to reach $(g_x, g_y)$ for all possible inputs.

### Solution

We first recognise that this can be written in a matrix equation of the form $A\boldsymbol{x}=b$.

That is, given the conventions above, we can write the matrix equation

$$\begin{bmatrix} x_1 & x_2 \\\ y_1 & y_2 \end{bmatrix} \begin{bmatrix} a \\\ b \end{bmatrix} = \begin{bmatrix} g_x \\\ g_y \end{bmatrix}$$

#### Non-singular Case

**Theorem 13.1.** *If $`A`$ is an invertible square matrix, then there exists a unique solution $`\boldsymbol{x}`$ such that $`A\boldsymbol{x}=B`$.*

*Proof.* Let $A$ be an invertible $n \times n$ square matrix. Then, by definition, there exists a matrix $A^{-1}$ such that $A^{-1} A = I_n$. 

Then, left-multiplying both sides of the equation $A\boldsymbol{x} = B$ yields $A^{-1} A\boldsymbol{x} = A^{-1} B$. This simplifies to $I_n \boldsymbol{x} = A^{-1} B$, which by definition of $I_n$ is equivalent to $\boldsymbol{x} = A^{-1} B$.

This is unique since suppose there exists another $\boldsymbol{x_2}$ that is a solution to the equation, then it must fulfil $A\boldsymbol{x_2} = B$. Left-multiplying both sides by $A^{-1}$, we get $\boldsymbol{x_2} = A^{-1} B$ since the identity matrix, $I_n$ is unique. This is equivalent to $\boldsymbol{x}$, so both are equivalent and therefore there must be a unique solution. $\blacksquare$.

We can therefore obtain the solution for $\boldsymbol{x}$.

$$ 
\begin{aligned}
\begin{bmatrix} a \\\ b \end{bmatrix} &= \begin{bmatrix} x_1 & x_2 \\\ y_1 & y_2 \end{bmatrix}^{-1} \begin{bmatrix} g_x \\\ g_y \end{bmatrix}\\\
&= \frac{1}{x_1y_2 - x_2y_1} \begin{bmatrix} y_2 & -y_1 \\\ -x_2 & x_1 \end{bmatrix} \begin{bmatrix} g_x \\\ g_y \end{bmatrix}\\\
&= \frac{1}{x_1y_2 - x_2y_1} \begin{bmatrix} y_2g_x - y_1g_y \\\ x_1g_y-x_2g_x \end{bmatrix}
\end{aligned}
$$

Therefore, we can conclude that:

$$
\begin{aligned}
a &= \frac{y_2g_x - y_1g_y}{x_1y_2 - x_2y_1} \\\ 
b &= \frac{x_1g_y-x_2g_x}{x_1y_2 - x_2y_1}
\end{aligned}
$$

However, since we can only press the buttons an integer number of times, we must check that the numerator of both must be divisible by the denominator (the determinant of the matrix). Furthermore, they must be positive.

#### Singular Case

However, there is an important edge case (which does not seem to appear in the inputs), where the matrix is singular. That is, the matrix $`\begin{bmatrix} x_1 & x_2 \\\ y_1 & y_2 \end{bmatrix}`$ has a zero determinant. 

This means that the matrices $`\begin{bmatrix} x_1 \\\ y_1 \end{bmatrix}`$ and $`\begin{bmatrix} x_2 \\\ y_2 \end{bmatrix}`$ are linearly dependent and therefore any linear combination of them does not span the whole of $\mathbb{R}^2$.

For such a matrix equation, there are either no solutions, or infinitely many solutions. 

We will first establish if there exists any solutions for this edge case. The check for this is simple, we simply need to check if the target vector is in the column space of our matrix. We can simply note that if $\frac{x_1}{y_1} = \frac{x_2}{y_2} = \frac{g_x}{g_y}$, then there exists a real solution to this equation. The first equality is trivially satisfied since the matrix is singular.

Now, suppose there exists real solutions, we now need to check for integer solutions to the equation. This can be done by performing the extended euclidean algorithm. We can simply ignore the second equation, since it is equivalent to the first.

First, we convert the equations to satisfy Bézout's identity. 

**Theorem 13.2. (Bézout's Identity)** *Let $`a`$ and $`b`$ be integers with greatest common divisor $`d`$. Then there exist integers $`x`$ and $`y`$ such that $`ax + by = d`$. Moreover, the integers of the form $`az + bt`$ are exactly the multiples of $`d`$.*

By Bézout's Identity, we know that there exists integers $a, b$ such that $\lambda x_1 + \mu x_2 = \gcd(x_1, x_2)$. Then, we can multiply the whole equation by $\frac{g_x}{\gcd(x_1, x_2)}$ to get:
$$x_1 \left(\lambda \cdot \frac{g_x}{\gcd(x_1, x_2)}\right) + x_2 \left(\mu \cdot \frac{g_x}{\gcd(x_1, x_2)}\right) = g_x.$$

This corresponds to the original equation $a x_1 + b x_2 = g_x$, where $a = \lambda \cdot \frac{g_x}{\gcd(x_1, x_2)}$ and $b = \mu \cdot \frac{g_x}{\gcd(x_1, x_2)}$.

Hence, a solution to the equation $ax_1 + bx_2 = g_x$ is $(x, y) = (\lambda \cdot \frac{g_x}{\gcd(x_1, x_2)}, \mu \cdot \frac{g_x}{\gcd(x_1, x_2)})$, where $(\lambda, \mu)$ is the solution to the corresponding equation $\lambda x_1 + \mu x_2 = \gcd(x_1, x_2)$.

#### Extended Euclidean Algorithm

We will first explain how to obtain $(\lambda, \mu)$. This can be done using the extended Euclidean algorithm.

First, we obtain $\gcd(x_1, x_2)$ using the standard Euclidean algorithm.

We start off with $r_0 = x_1, r_1 = x_2$, with $x_1>x_2$.

Then, we define $r_i = r_{i-2} - q_ir_{i-1}$, where $q_i$ is the quotient at step $i$ and $r_i$ is the remainder at step $i$. Essentially, we divide $r_{i-2}$ by $r_{i-1}$, and obtain the pair of quotient and remainder $(q_i, r_i)$. We note that this eventually terminates when the remainder is $r_n = 0$, and $r_{n-1} = \gcd(x_1, x_2)$.

We will show that this holds.

**Lemma 13.1.** *If $`a = bq+r`$, where $`q`$ is the quotient and $`r`$ is the remainder after dividing $`a`$ by $`b`$, then $`\gcd(a,b) = \gcd(b,r)`$.*

*Proof.* If $a = bq+r$, where $q$ is the quotient and $r$ is the remainder after dividing $a$ by $b$, then we can write $r = a - bq$. 

Let $d$ be any common divisor of $a$ and $b$. Then, we can write $a = k_1 d, b= k_2 d$. We then substitute $a$ and $b$ into $r$.

$$
\begin{aligned}
r &= a - bq\\\
&= k_1 d - k_2 d q\\\
&= (k_1 - k_2) d q
\end{aligned}
$$

Therefore, since $q$ is an integer, $d$ is a divisor of $r$ as well. Since $\gcd(a,b)$ is a divisor of $a$ and $b$, it must also be a divisor of $b$ and $r$. Since successive $r$ values are decreasing, this must eventually terminate.

Once we have obtained the value of $\gcd(x_1, x_2)$, we now need to find the values of $a$ and $b$ such that $ax_1 + bx_2 = \gcd(x_1, x_2)$. We will then reverse the Euclidean algorithm to obtain these values. Recall that we started off with $r_0 = x_1$ and $r_1 = x_2$. 

We start off with the final non-zero remainder from the Euclidean algorithm,$r_{n-1} = \gcd(x_1, x_2)$.

We have $r_{n-1} = r_{n-3} + q_{n-1} r_{n-2}$. Note that since we have the values of $r_i$ and $q_i$, we can substitute $r_i = r_{i-2} + q_i r_{i-1}$ until we reach $r_0$ and $r_1$. Thereafter, we will have expressed $r_{n-1} = \lambda r_0 + \mu r_1$, which is equivalent to $\gcd(x_1,x_2) = \lambda x_1 + \mu x_2$, and we would have obtained a value for $\lambda$ and $\mu$.

Now that we have a solution to $\lambda x_1 + \mu x_2 = \gcd(x_1,x_2)$, we can find the values of $(a, b)$ in the original equation.

#### Finding All Other Solutions

Now suppose that we have $a_0x_1 + b_0x_2 = g_x$ as a solution, and that $ax_1 + bx_2 = g_x$ is another solution.

Then, subtracting both equations, we get
$$x_1(a_0 - a) + x_2(b_0 - b) = 0 \Rightarrow x_1 (a_0 - a) = x_2(b - b_0).$$

This means that $x_1$ divides $x_2(b-b_0)$, which also implies that $\frac{x_1}{\gcd(x_1,x_2)}$ divides $b-b_0$. We will show why this is true below:

There must exist an integer $k = (a_0 - a)$ such that $x_2(b-b_0) = x_1 \cdot k$. Let $g = \gcd(x_1, x_2)$, then we can write $x_1 = g \cdot \frac{x_1}{g}, x_2 = g \cdot n$, where $\frac{x_1}{g}$ and $n$ are coprime by definition.

Substituting, we have $g\cdot n \cdot (b-b_0) = g \cdot \frac{x_1}{g} \cdot k$. Dividing, we have $n \cdot (b-b_0) = \frac{x_1}{g} \cdot k$. Recall that $\frac{x_1}{g}$ and $n$ are coprime, thus, $n$ cannot be divisible by $\frac{x_1}{g}$. Hence, it must be true that $b-b_0$ is divisible by $\frac{x_1}{g}$.

Therefore, $b - b_0 = r \cdot \frac{x_1}{\gcd(x_1,x_2)}$ for some positive integer $r$, or $b = b_0 + r \cdot \frac{x_1}{\gcd(x_1,x_2)}$. Substituting this into $a$ and $a_0$, we obtain the pair of equations:

$$
\begin{aligned}
a &= a_0 - r \cdot \frac{x_2}{\gcd(x_1,x_2)}\\\
b &= b_0 + r \cdot \frac{x_1}{\gcd(x_1,x_2)}
\end{aligned}
$$

Without loss of generality, we can replace $r$ with $-r$ to obtain the nicer equations:

$$
\begin{aligned}
a &= a_0 + r \cdot \frac{x_2}{\gcd(x_1,x_2)}\\\
b &= b_0 - r \cdot \frac{x_1}{\gcd(x_1,x_2)}
\end{aligned}
$$

We now want to find the minimum $(a, b)$ in order to minimize costs, such that $a, b\geq 0$. 

We will find the pair of solutions that have minimal $|a|$ (and consequently $|b|$), because if neither of these pair of solutions have both $a, b>0$, then no other solution will.

This means that we will first find $k_0 = -\frac{a}{x_1 / \gcd(a,b)}$. The other pair will be given by $k_1 = k_0 + 1$. We then check if either of this pair of solutions is valid. If so, we return it.

### Code Complexity

**Time Complexity:** $O(T \times \log(min(x_1, x_2)))$

* $T$ is the number of testcases.
* $x_1$, $x_2$ are the values of $X$ for button A and B respectively. 

Calculating the determinant is a constant time operation. 

In the case of a non-singular matrix, computing the inverse matrix involve elementary arithmetic operations and checking for divisibility and calculating the cost are all constant time operations. So, this branch has a time complexity of $O(1)$.

For the singular matrix case, checking if the prize can be reached is a constant time operation. The extended Euclidean algorithm, however, takes $O(\log(min(x_1, x_2)))$ time. Scaling and computing the minimal solutions also takes $O(1)$ time. So, this branch has a time complexity of $O(1)$.

**Additional Space Complexity:** $O(T)$

All the testcases are stored in memory. Each testcase has 6 numbers.

**Final answer (Part A):** 33427.
**Final answer (Part B):** 91649162972270.