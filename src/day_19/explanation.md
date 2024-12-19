# Day 19: Linen Layout

## Input Bounds

Based on preliminary input analysis, all characters are lowercase alphabets.

## Part A

### Abridged Problem Statement

Given a set of patterns in the form of lowercase strings, and a set of designs in the form of lowercase strings, find the number of designs where the design can be formed by concatenating any of the patterns. Each pattern can be used multiple times.

### Solution

First, we establish a DP recurrence. Let $dp_i$ be if it is possible to form the design substring in the range $[0, i]$ by concatenating any of the patterns.

We notice that there are two criteria:

* For any pattern of length $x$, it must already be possible to construct the design substring in the range $[0, i-x]$.
* The design substring in the range $[i-x+1, i]$ must match the pattern.

Therefore, we can express the DP recurrence as, for a given list of patterns $P$, and $|p|$ representing the length of the pattern $p$, and to match the design $d$:

$$dp_i = \bigvee_{p \in P} (d[i - |p| + 1, i] == p) \land dp_{i-|p|}$$

We note that the bottleneck of this would be iterating through all patterns for every index. We can therefore optimize this using a trie.

#### Trie

A trie has the root node represent the empty string, and every child contains will share its prefix with its parent. An example for building the trie on the sample input is shown below:

![trie](https://kroki.io/tikz/svg/eNp9k8tugzAQRfd8xTRSpFRKaehzUbHqH3QbssD2hFjxAxlTlCD_e-0474ZsLDNzONwxcsE0bSUqS0XZNHPL19sp0YahyV_T7E3KRd_YUrFSaIUuKdoG65Kuywr7wLokKQhWXPUHz6kS-jWntjXo5gkAsgphabSEujSefE4buxGY974HwEzZTXe7p8ZiKewqPtkVp-uwdWHBXzQbUJrh0NuUGyow7iVXXLYSGr7F_EPKWOVKoQE_R57V9mimWtYC7ZV3yYXIiWjxIZsdSOFDCMiOYMOJ4KoCxsNJUcyz9J3KM_RlGJ2ln5FdJMkYfrS2u-GSIqzzMNUCJsaXH6F3IeWKCwb9rntIHAjfNi7OP0AQjxDnYAzf-yp0_j_DyJBRmOxSDpPO891dZRe_ekPZmZvKGOGekgwqyU55LTwLOezsWk-1N6VdG63uf9xzReUF1d3g1eDxVvF4v_y1QMUuLsW-dLo5f0WOKSE=)

In particular, the blue circles indicate the word endings. Therefore, we note that for our trie nodes, we need to store both a pointer to their children nodes, and whether the node is the end of a word. Therefore, the trie node will look like:

```cpp
struct TrieNode {
    TrieNode* children[26];
    bool is_word;

    TrieNode() {
        is_word = false;
        for (int i=0; i<26; i++) {
            children[i] = nullptr;
        }
    }
}
```

Now, to insert a word in the trie, we simply need to determine whether the current character exists as a child of the current node in the trie. Otherwise, we create it. Then, we will set our current pointer to that character. This will look like:

```cpp
void insert(TrieNode* root, string &word) {
    TrieNode* curr = root;
    for (char c : word) {
        if (curr -> children[c - 'a'] == nullptr) {
            curr -> children [c - 'a'] = new TrieNode();
        }
        curr = curr -> children[c - 'a'];
    }
    curr -> is_word = true;
}
```

Note that at the end of each word, we will set the `is_word` flag to true.

Searching in a trie involves performing a DFS on the trie. If we ever come across a null pointer for the trie, we will know that the word does not exist in the trie.

#### Optimizing DP solution with trie

We look at the DP recurrence, for a given list of patterns $P$, and $|p|$ representing the length of the pattern $p$, and to match the design $d$:

$$dp_i = \bigvee_{p \in P} (d[i - |p| + 1, i] == p) \land dp_{i-|p|}$$

We note that we can speed up the transition by simply iterating from the last valid index, $i - |p|$ and make use of the trie to check whether the next character is a valid pattern. If we ever come across a character which is an invalid pattern, we can break out of the iteration. If we come across the end of a word at index $j$, as indicated by the `is_word` flag, then it is possible to construct a word at index $j$. We can then set $dp_j$ as true.

Hence, we can rewrite the DP transition as:

$$dp_j = dp_i \land \text{is\_word}_j$$

The naive DP solution above has a time complexity of $O(|P| \times \overline{|p|})$ for each index, where $|P|$ is the number of patterns, and $\overline{|p|}$ is the mean length of each pattern.

This trie-based solution improves the time complexity for checking each index to $O(|d|)$, where $|d|$ is the length of the design.

At the end of the iteration, we simply check if the design can be constructed by checking whether $dp_{|d|-1}$ is true.

### Code Complexity

**Time Complexity:** $O(D \times d^2 + P \cdot p)$

* $D$ is the number of designs.
* $d$ is the average length of the design.
* $P$ is the number of patterns.
* $p$ is the average length of the patterns.

It takes a time of $O(O \times p)$ to construct the trie. However, this construction only needs to be done once for all the designs.

For each design, there is a time complexity of $O(d^2)$ to check whether it can be constructed by the patterns.

**Additional Space Complexity:** $O(P \times p + d)$

The trie requires a construction space of $O(P \times p)$ in the worst case scenario. The DP array takes up a size of $O(P)$.

**Final answer:** 311.

## Part B

### Abridged Problem Statement

Given a set of patterns in the form of lowercase strings, and a set of designs in the form of lowercase strings, find the sum of the number of ways to construct each design by concatenating any of the patterns. Each pattern can be used multiple times.

### Solution

We can use a similar idea as the DP solution above. Instead, we now let $dp_i$ be the number of ways to construct each design substring $[0, i]$.

We can rewrite the DP transition as:

$$dp_j = \sum_{i=0}^{j}\left\{\begin{array}{ll}
dp_i & dp_i \land \text{is\_word}_j\\
0 & \text{Otherwise}
\end{array}\right.$$

The number of ways to construct a design, $d$, is therefore $dp_{|d|-1}$.

### Code Complexity

**Time Complexity:** $O(D \times d^2 + P \cdot p)$

* $D$ is the number of designs.
* $d$ is the average length of the design.
* $P$ is the number of patterns.
* $p$ is the average length of the patterns.

It takes a time of $O(O \times p)$ to construct the trie. However, this construction only needs to be done once for all the designs.

For each design, there is a time complexity of $O(d^2)$ to check whether it can be constructed by the patterns.

**Additional Space Complexity:** $O(P \times p + d)$

The trie requires a construction space of $O(P \times p)$ in the worst case scenario. The DP array takes up a size of $O(P)$.

**Final answer:** 616234236468263.
