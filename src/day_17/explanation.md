# Day 17: Chronospatial Computer

## Input Bounds

Based on preliminary data analysis, the input values all fit in a 32-bit unsigned integer.

## Part A

### Abridged Problem Statement

You are given an input of the initial value of three registers, A, B and C.

You are then given a series of pairs of opcodes and operands, comma separated.

There are two kinds of operands, literal and combo operands:
* **Literal operands:** This are numbers from 0 to 7, and represent the value itself.
* **Combo operands:** For operands from 0 to 3, these represent the value itself. 4, 5 and 6 represent the value in register A, B and C respectively. 7 is not a valid combo operand.

Each opcode has the following instructions:
* **0, 6, 7:** First, find the result of dividing the value in register A by $2^x$, where $x$ is the value of the combo operand. Store the result in register A, B and C respectively.
* **1:** Find the result of the bitwise XOR of the value in register B and the literal operand. Store the result in register B.
* **4:** Find the result of the bitwise XOR of the value in register B and the value in register C. Store the result in register B.
* **2:** Find the result of the combo operand modulo 8. Store the result in register B.
* **3:** Jump to the position indicated by the literal operand if the value in register A is not 0. Otherwise, ignore it.
* **5:** Output the value of the combo operand modulo 8.

The program terminates once the pointer for instructions moves beyond the last instruction in the list.

Find the output of the programs, comma separated.

### Solution

Simply simulate the program by implementing the operations above.

### Code Complexity

**Time Complexity:** $O(N \times R)$

* $N$ is the number of instructions.
* $R$ is the number of times each instructions will be repeated (due to the jump instruction).

The jump instruction might alter the flow of instructions because it might jump to an earlier instruction.

**Additional Space Complexity:** $O(N)$

The instructions are stored in a vector.

**Final answer:** 2,7,4,7,2,1,7,5,1.

## Part B

### Abridged Problem Statement

Given the above program instructions and the input of the program, find the register value A such that the input register value A will generate an output that is identical to the input of the program.s

### Solution

Based on my input, the instructions are equivalent to, where $a$, $b$, $c$ are the values in the A, B and C register respectively:

```
do {
    b = a % 8;
    b ^= 2;
    c = a >> b;
    b ^= c;
    b ^= 3;
    output(b);
    a = a >> 3;
} while a != 0
```

We can express the value of $b$ in terms of $a$ only.

$b = (((a \bmod 8) \oplus 2) \oplus \frac{a}{2^{(a \bmod 8) \oplus 2}} \oplus 3) \bmod 8 $

Afterwards, the value of $a$ is bitshifted to the right by 3 positions. Effectively, the output $b$ only represents the last 3 positions, and the rest of the value of $a$ is then processed.

Hence, since we know that the last value is 0, we can start with finding the value of $a$ such that $b$ matches the program instructions, since $a$ can only range from $0$ to $7$. We can then test the value of $b$ against the program instructions, and store all the valid values.

We will then add this value to the value of all previously obtained values, bitshifted to the left by 3 bits.

At the end of the program, we iterate through this set of numbers to find the minimum.

### Code Complexity

**Time Complexity:** $O(N \times M)$

* $N$ is the number of instructions.
* $M$ is the average number of numbers obtained after each iteration.

For each instruction value, we need to iterate through all the previously obtained possible values and add their bitshifted value to the new valid values that will give rise to the current instruction value. We need to test at most 8 such new values.

**Additional Space Complexity:** $O(8^N)$

In the worst case scenario, we will have $8^N$ different numbers that are possible (if all are possible).

**Final answer:** 37221274271220.
