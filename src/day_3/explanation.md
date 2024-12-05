# Day 3: Mull It Over

## Input Bounds

From preliminary inspection, the input is a multi-line string that contains **mul($a$,$b$)** in it. $a$ and $b$ appear to be unsigned 3 digit numbers, so their product will fit in a 32-bit unsigned integer.

## Part A

### Abridged Problem Statement

Given a multi-line string that contains **mul($a$,$b$)**, where $a$ and $b$ are integers, extract all such substrings. Sum all occurrences of $a\times b$.

### Solution

Simply use a Regex of the form:
```
mul\((\d+),(\d+)\)
```

* `mul\(` matches for an exact match of `mul(`.
* `(\d+)`,`(\d+)` introduces two matching groups, which must be separated exactly by a comma, and must contain 2 integers.
* `\)` matches for an exact match of `)`.

### Code Complexity

**Time Complexity:** $O(L \times M \times N^2)$

* $L$ is the number of lines of string.
* $M$ is proportional to the size of the regex.
* $N$ is proportional to the size of the string being searched.

**Additional Space Complexity:** $O(M)$

**Final answer:** 174103751.

## Part B

### Abridged Problem Statement

Given a multi-line string that contains **mul($a$,$b$)**, where $a$ and $b$ are integers, extract all such substrings. Sum all occurrences of $a\times b$.

However, there are now `do()` and `don't()` modifiers, which enable and disable the `mul()` function respectively. 
* `do()` enables all `mul()` operations to the right of it, until either the end of the string or until the next modifier.
* `don't()` disables all `mul()` operations to the right of it, until either the end of the string or until the next modifier.
* By default, `mul()` operations are enabled.

### Solution

We will use a regex of the form
```
(?:(do\(\))|(don't\(\))|(?:mul\((\d+),(\d+)\)))
```

* `(do\(\))` is the first capturing group, matching exactly `do()`.
* `(don't\(\))` is the second capturing group, matching exactly `don't()`.
* `(?:mul\((\d+),(\d+)\))` contains the third and fourth capturing group, which match the numbers to be multiplied as per part A.

We will then use a match to determine which capturing group it belongs to, then have a boolean flag for whether the `mul()` function is enabled or disabled.

### Code Complexity

**Time Complexity:** $O(L \times M \times N^2)$

* $L$ is the number of lines of string.
* $M$ is proportional to the size of the regex.
* $N$ is proportional to the size of the string being searched.

**Additional Space Complexity:** $O(M)$

**Final answer:** 100411201.
