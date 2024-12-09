# Day 9: Disk Fragmenter

## Input Bounds

From preliminary data analysis, the input is simply a very long string with numeric characters, inclusive of 0.

## Part A

### Abridged Problem Statement

You are given a disk map, which is a string of numbers. The disk is alternates between memory blocks and free space. Each digit of the disk map switches between indicating the length of a memory block and the length of some free space, starting with the length of a memory block. A length 0 blocks is also valid.

Each memory block also has an ID, which is 0-indexed and depends on its original position before anything is rearranged.

You move each memory block one at the time, starting from the rightmost memory block, moving it to the leftmost block of free space, until there are no more gaps between file blocks.

Lastly, you need to compute the *checksum* after this movement. Let the memory block ID at position $i$ (0-indexed) be $f_i$ after rearrangement. Then, the checksum $C$ is given by

$$C = \sum_{i=0}^n i \cdot f_i$$

### Solution

Use a two-pointer approach, and maintain the current starting index if the string were to be expanded. When you encounter:

* An occupied memory chunk

Calculate the checksum of this memory chunk by determining the current starting index and its ending index. Then, simply calculate the sum of the consecutive integers between these two indices, and multiply it by the file ID.

That is, if the starting index is $x$, and the length of the memory chunk is $l$, and the file ID is $d$, then the checksum of this segment is given by 

$$C = \frac{(x+l)(l-x + 1)}{2} \cdot d$$

* A chunk of free space

We will then move our right pointer leftwards until we meet a chunk of occupied memory. There are two cases:

1. This chunk of occupied memory can fit completely in our chunk of free space.

In which case, we move it to the chunk of free space and calculate the checksum as per above. We then continue moving leftwards until we meet another chunk of occupied memory, or until we meet the left pointer.

2. This chunk of occupied memory cannot completely fit in our chunk of free space.

Then, we move as much memory as possible to the free space and calculate the checksum. We then reduce the amount of occupied memory we need to move by the amount of free space available, and move on to moving the left pointer again.

Lastly, we make a note on how to efficiently get the memory chunk ID from its position in the original string. We note that the ID of a memory chunk in position $i$ will be $\frac{i}{2}$, since the memory chunks are all in even positions (0-indexed).

### Code Complexity

**Time Complexity:** $O(N)$

* $N$ is the number of characters in the string.

We use a two-pointer approach, so each character in the string will be processed at most once.

**Additional Space Complexity:** $O(N)$

We need to store the whole string in a vector.

**Final answer:** 6463499258318.

## Part B

### Abridged Problem Statement

To be completed.

### Solution

### Code Complexity

**Time Complexity:** $O(N \log N)$

* $N$ is the number of characters in the string.

**Additional Space Complexity:** $O(N)$

**Final answer:** 6493634986625.
