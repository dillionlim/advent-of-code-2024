# Day 24: Crossed Wires

## Input Bounds

Based on preliminary data analysis, the first section contains three lowercase alphanumeric characters. The second number is either `0` or `1`.

The second section contains a string which consists of three lowercase alphanumeric characters, followed by `XOR`, `OR` or `AND`, followed by another string which consists of three lowercase alphanumeric characters. It is then followed by a `->`, and another string which consists of three lowercase alphanumeric characters.

## Part A

### Abridged Problem Statement

Given the input as described above, the first section contains the binary values of codes. 

The second section contains the mapping of codes. It is in the form of `A OP B -> C`, where the value of `C` is given by the operator described, applied to `A` and `B`. 

At the end, find all the codes corresponding to `zXX`, where `XX` is an integer. String them from `z00` to the largest index, where `z00` is the least significant bit of the number. Find the decimal representation of this number.

### Solution

We note that we can form a tree based on the dependencies of a given code. `C` will have children `A` and `B` as dependencies. Then, the initial values for the codes given will be the leaf nodes.

We can therefore recursively compute the value of `zXX`. We will store all initial codes in a hashmap, and whenever we encounter the value of the code in the hashmap, we will return it. Otherwise, we will recursively compute the values of its children nodes. Then, we will memoize the value of this code in the hashmap to prevented repeated computations.

At the end, we iterate over all values of `z00` to the maximum `zXX` value in reverse. Then, at the start of each iteration, we will bitshift the answer to the left by one, then we will add the value to the current answer.

### Code Complexity

**Time Complexity:** $O(N)$

* $N$ is the total number of codes in the list.

Each code will have its value computed at most once due to the memoization.

**Additional Space Complexity:** $O(N)$

All the codes will eventually have their value stored in the hashmap.

**Final answer:** 69201640933606.

## Part B

### Abridged Problem Statement

The input actually represents an adder, where the values of `x00` to `xXX`, where `xXX` is the most significant bit represent value $a$, and the values of `y00` to `yYY`, where `yYY` is the most significant bit represent value $b$, and the output of `z00` to `zZZ` represent the value of $a+b$. However, the outputs codes of some of the mappings are swapped. It is known that the output codes will only be swapped once, so swapped codes will always be swapped in pairs.

Find all swapped codes, and output them space separated, ordered in lexicographically increasing order.

### Solution

By analysing the general dependency graph, we notice that the addition process is quite recursive. This hints at it being a ripple-carry adder.

This means that the circuit will be of the form:
![rca](rca.png)
*Image taken from [Wikipedia](https://upload.wikimedia.org/wikipedia/commons/5/5d/4-bit_ripple_carry_adder.svg).*

In particular, the full adder will be of the form:
![fa](full-adder.png)
*Image taken from [Wikipedia](https://en.m.wikipedia.org/wiki/File:Full-adder.svg).*

Therefore, we can establish a few rules to look for incorrect output values:
* Each `A XOR B -> C` mapping can fall into two possible categories:
    * **Input XOR mappings (`xAA XOR yAA -> int`):** This should therefore not output anything starting with `zXX`, unless `AA` is 00, in which case, the first adder is a half-adder. Therefore, `x00 XOR y00 -> z00`. 
    * **Output sum XOR mappings (`int XOR Cin -> S`):** Since for a full adder, $S = A \oplus B \oplus C_{in}$, where $A, B$ are the input values and $C_{in}$ is the carry input value, and we have $A \oplus B$ from the previous XOR type, this XOR mapping type will take a previous intermediate value and XOR a carry intermediate value to output something of the form `zXX`.
* We can therefore establish a few rules from these two types of XOR mappings:
    * All XOR gates that input `xXX` and `yXX` cannot output `zXX`, except for `x00` and `y00`.
    * All other XOR gates must output `zXX`.

        The above two rules will identify all the incorrect outputs from XOR gates. We will now need to find the other pair from this XOR gate.

    * All gates that output `zXX` must be an XOR gate. The only exception is the final `zXX` value, which will be the final carry bit, and therefore will be the output of a OR gate.
    * All gates that input `xXX` and `yXX` must output to a corresponding valid code that outputs to `zXX`. That is, if `xXX XOR yXX -> aaa`, then there must exist a mapping `aaa XOR bbb -> zXX`.

        In such a case, if a mismatch is found, we will then need to identify the other mismatched value in the pair. We note that `xXX XOR yXX` must eventually lead to `zXX`, therefore, we can simply look for the actual children of `zXX` in the mapping. Let these children be $a$ and $b$.
        
        From the logic diagram, we note that one of the inputs come from the $C_{in}$ values, and the other comes from the `xXX XOR yXX` mapping, which now has an incorrect value. We know that the value of $C_{in}$ is correct, so we must then look for the corresponding `OR` statement. 

        Suppose, without loss of generality that $a$ is the child with a corresponding `OR` statement. $b$ must then be the child which is the correct output of our original `xXX XOR yXX` mapping. We have then found the pairing that was swapped.

### Code Complexity

**Time Complexity:** $O(M)$

* $M$ is the number of rules.

In the worst case, we will iterate over all rules to find the 8 incorrect values. The verification of each rule involves $O(1)$ checks.

**Additional Space Complexity:** $O(M)$

All rules need to be stored for verification.

**Final answer:** dhq,hbs,jcp,kfp,pdg,z18,z22,z27.
