# Day 2a: Red-Nosed Reports

Fortunately, the first location The Historians want to search isn't a long walk from the Chief Historian's office.

While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no sign of the Chief Historian, the engineers there run up to you as soon as they see you. Apparently, they still talk about the time Rudolph was saved through molecular synthesis from a single electron.

They're quick to add that - since you're already here - they'd really appreciate your help analyzing some unusual data from the Red-Nosed reactor. You turn to check if The Historians are waiting for you, but they seem to have already divided into groups that are currently searching every corner of the facility. You offer to help with the unusual data.

The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

```
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
```

This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

* The levels are either all increasing or all decreasing.
* Any two adjacent levels differ by at least one and at most three.

In the example above, the reports can be found safe or unsafe by checking those rules:

* **7 6 4 2 1:** Safe because the levels are all decreasing by 1 or 2.
* **1 2 7 8 9:** Unsafe because 2 7 is an increase of 5.
* **9 7 6 2 1:** Unsafe because 6 2 is a decrease of 4.
* **1 3 2 4 5:** Unsafe because 1 3 is increasing but 3 2 is decreasing.
* **8 6 4 4 1:** Unsafe because 4 4 is neither an increase or a decrease.
* **1 3 6 7 9:** Safe because the levels are all increasing by 1, 2, or 3.
So, in this example, 2 reports are safe.

Analyze the unusual data from the engineers. How many reports are safe?

## Abridged Problem Statement

You have $Q$ queries. 

Each query consists of a list of $N$ numbers. Each query is *safe* if it is either strictly decreasing or strictly increasing, and adjacent numbers differ by a value between 1 and 3 (inclusive). 

Count the number of *safe* queries.

## Solution

### Input Bounds

From preliminary data analysis, we know that the numbers are all unsigned 32-bit integers.

We also know that each row must have at least 2 integers, but we will not use that fact here (we will still handle it).

### Actual Solution

* If the row only has 1 number, it is safe trivially.
* Otherwise, we will establish whether the row is increasing or decreasing by examining the first 2 numbers of the row. 
* Based on the established direction, we will then examine the rest of the numbers to determine if they fall within the acceptable range (1 to 3).
* This can be implemented quite easily in Rust with a sliding window (`array.window(2)`).

### Code Complexity

**Time Complexity:** $O(N \times M)$

* N is the number of reports
* M is the number of levels

Trivially, each number is looked at only once.

**Additional Space Complexity:** $O(1)$

A constant amount of extra space is used.

**Final answer:** 321.
