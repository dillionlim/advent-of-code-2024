# Day 2b: Red-Nosed Reports

The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.

The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!

Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

More of the above example's reports are now safe:

* **7 6 4 2 1:** Safe without removing any level.
* **1 2 7 8 9:** Unsafe regardless of which level is removed.
* **9 7 6 2 1:** Unsafe regardless of which level is removed.
* **1 3 2 4 5:** Safe by removing the second level, 3.
* **8 6 4 4 1:** Safe by removing the third level, 4.
* **1 3 6 7 9:** Safe without removing any level.
Thanks to the Problem Dampener, 4 reports are actually safe!

Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe?

## Abridged Problem Statement

You have $Q$ queries. 

Each query consists of a list of $N$ numbers. Each query is *safe* if it is either strictly decreasing or strictly increasing, and adjacent numbers differ by a value between 1 and 3 (inclusive), or if it has at most 1 number that can be removed to make the query safe.

Count the number of *safe* queries.

## Solution

Anything less than 3 numbers will be trivially safe, and for 3 numbers, a check simply needs to be done to check if any 2 numbers will form a valid sequences.

For a pair of numbers $(a_i, a_{i+1})$ that fails either condition above, if the removal of at least 1 of them can resolve the problem, then it is possible to make the query safe. We only need to test the first violation that occurs, since subsequent violations do not receive this treatment.

However, there is a chance that the removal of $a_i$ will flip the direction of the sequence, e.g. 8 9 8 7 6, which will flip it from increasing to decreasing and make it valid. So this possibility must be considered as well. Therefore, in general, testing the first 4 numbers is sufficient to make a decision on the direction of the sequence (with majority winning). 

**Final answer:** 386.
