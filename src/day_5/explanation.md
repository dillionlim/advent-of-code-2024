# Day 5: Print Queue

## Input Bounds

From preliminary data analysis, it appears that all the page numbers are unsigned 32-bit integers. Furthermore, the set of page numbers forms a directed acyclic graph. It is also guaranteed that the update lists have odd length.

## Part A

### Abridged Problem Statement

Given a series of rules, $R = \{(A,B) | A|B \Longrightarrow A \prec B\}$, define the strict poset $P = (X, \prec)$, where $X = \displaystyle\bigcup_{(A,B)\in R} \{A,B\}$, the set of elements in $R$.

For a series of updates, $U \subseteq X$, the induced subposet $P_U = (U, \prec_U)$ is defined, and $\prec_U$ is the restriction of $\prec$ to $U$. An update is *ordered* if the given linear ordering of $U$ is a valid linear extension of $\prec_U$.

Find the sum of the middle element of all *ordered* updates.

### Solution

Since it is guaranteed that there exists a linear ordering of the pages, then we can simply do a linear check on the pages. 

That is, we store all the rules in a hashset. For each element $U_x$, we determine if for the next element, $U_{x+1}$, the rule $U_x | U_{x+1}$ exists in the hashset. If the entire update fulfils the requirements, then the update is *ordered*. We can then find the middle element.

### Code Complexity

**Time Complexity:** $O(L + M \times N)$
* $L$ is the number of rules
* $M$ is the number of updates
* $N$ is the average number of elements in each update.

We first need to do $O(L)$ preprocessing for each rule by storing the pair of numbers in a hashset. Then, for each update, checking whether an update is *ordered* involves a single pass through the update, which takes $O(N)$ time.

**Additional Space Complexity:** $O(L)$

All the rules need to be stored in the hashset.

**Final answer:** 6612.

## Part B

### Abridged Problem Statement

Given a series of rules, $R = \{(A,B) | A|B \Longrightarrow A \prec B\}$, define the strict poset $P = (X, \prec)$, where $X = \displaystyle\bigcup_{(A,B)\in R} \{A,B\}$, the set of elements in $R$.

For a series of updates, $U \subseteq X$, the induced subposet $P_U = (U, \prec_U)$ is defined, and $\prec_U$ is the restriction of $\prec$ to $U$. An update is *ordered* if the given linear ordering of $U$ is a valid linear extension of $\prec_U$.

Find the sum of the middle element of all *unordered* updates, after they are rearranged to become *ordered*.

### Solution

We will adapt the solution for part A to find all the *unordered* updates.

For the *unordered* updates, we need to find the middle element after it is *ordered*. We can simply sort the pages based on their partial ordering, as $a \prec b$ if $a|b$ is a rule. This, of course, requires all pairs of numbers to have their orderings to be defined, which is true in this case.

### Code Complexity

**Time Complexity:** $O(L + M \times N \log N)$

* $L$ is the number of rules
* $M$ is the number of updates
* $N$ is the average number of elements in each update.

We first need to do $O(L)$ preprocessing for each rule by storing the pair of numbers in a hashset. Then, for each update, checking whether an update is *ordered* involves a single pass through the update, which takes $O(N)$ time. However, in the worst case where all the updates are *unordered*, then we would need to sort all of the updates, which each take $O(N \log N)$ time.

**Additional Space Complexity:** $O(L)$

All the rules need to be stored in the hashset.

**Final answer:** 4944.
