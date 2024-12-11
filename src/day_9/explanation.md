# Day 9: Disk Fragmenter

## Input Bounds

From preliminary data analysis, the input is simply a very long string with numeric characters, inclusive of 0.

## Part A

### Abridged Problem Statement

You are given a disk map, which is a string of numbers. The disk is alternates between memory blocks and free space. Each digit of the disk map switches between indicating the length of a memory block and the length of some free space, starting with the length of a memory block. A length 0 block is also valid.

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

You are given a disk map, which is a string of numbers. The disk is alternates between memory blocks and free space. Each digit of the disk map switches between indicating the length of a memory block and the length of some free space, starting with the length of a memory block. A length 0 block is also valid.

Each memory block also has an ID, which is 0-indexed and depends on its original position before anything is rearranged.

You move each memory block one at the time, starting from the rightmost memory block, moving it to the leftmost block of free space **that can fit the entire block**. If no such blocks exists, do not move the memory block.

Lastly, you need to compute the *checksum* after this movement. Let the memory block ID at position $i$ (0-indexed) be $f_i$ after rearrangement. Then, the checksum $C$ is given by

$$C = \sum_{i=0}^n i \cdot f_i$$

### Solution

We note that for each memory block, we want to find the leftmost memory block that has a length longer than or equal to the length of the memory block. 

Let the list of lengths of the blocks of free space be $A$. Effectively, for a memory block of length $k$, we are looking for the leftmost number, $x$, such that $x \geq k$. Afterwards, we subtract $k$ from $x$.

In essence, this boils down to a set of query and update on the array $A$.

* **Query($\boldsymbol{k}$):** Find the smallest index, $i$ in $A$ such that $A_i \geq k$.
* **Update($\boldsymbol{i}, \boldsymbol{k}$):** Subtract $k$ from $A_i$.

We can perform both queries in $O(\log N)$ time by constructing a range-maximum segment tree.

For example, for the input $92765432423$, we can filter the memory blocks into $M = \{9,7,5,3,4,3\}$ and the spaces into $A = \{2,6,4,2,2\}$. We can then construct the segment tree on $A$ as follows:

![segment_tree](https://kroki.io/tikz/svg/eNqllU2L2zAQhu_-FVPYQAshTZzdUig5tNBDoYVSKD2s9yBLE0eNLJmRkjQ1_u8dx9iONx9tdw0xiqR5n3dGgzWC7x5hTW6tITiQzm6RQj3UeeEoiNQgGG3XUZQoJzc52iCN8L4UFLQ0WEXJxmMh5FpkWAa9_t3M1COjUxK0L_1KFOgnGbocA2k5iLkXMugtjoPOVqHgiYeyINxq3PG2r83oo91qcraGHxCFlmFDNZtBBm0WVu3WD44UUjmdzOK7IlRsO8VM27I1388cC91HAAa3aEBpH4SVuJhN3sp8DCP4ZCWh8Ki6NUgx7BBtE-I5FlWGsCSXQyGIKa8nPuwNLkpFYlfVKt8wd1vWEERut0KhDmFc7D1Yp7ALIJTMyAyOgdzGKg6RjiySH0MttkgN120MS23MIiOxfxFPxxDwVwDJYCRUzd9uo7NhkfhcGFONuzRnHdDrlM836xO_k_nB8Q_NhQTPx1QvLx0xk3xoBDqh-LJQPGmlvnCGJAIO1Dxyt6lHcvMrcizGWu_Vz40PXJZjrbBC_mlq5R6iiEvuXDgUN0rqN5QjxrSHL9KNEVSV3IzAz424n0IilQsebh9uIEkO07n4tXhTR6FVXUxUh4zgc1NIHsuVNgrKHnIRMwTFPahHPYYBNKEtMIaXBpcB_CYNhPjqsHjGwRUPQxezYxfHPk6dtF56N_OhG3BLOHV3wd9Vh0OP06HHxmXc65z4ZKfV89izC_UZVugqu3rq2cRnO6Qh314-m7ZZ_rMf52ca_7jCf-1Hqj_dz27Izsb8NOn4etJP4t2eTfufeNU7vkUOS8eXUTPVXzR_AF68To8=)

#### Query

To perform the query, we perform a technique known as **fractional cascading**.

Since we want to find the leftmost index, $i$, such that $A_i \geq k$, we will simply descend down the leftmost path that satisfies the inequlity above. 

Therefore, in order of priority,
* We simply check if the left node, $S_l \geq k$. If so, we descend down that path. 
* Otherwise, we descend down the right path if $S_r \geq k$. 
* If both paths are less than $k$, then we know that the entire array $A$ is less than $k$, and that no such numbers exist in the array.

In our example given above, taking the rightmost memory block, we need a free space with a length of at least 3. 

We first check if the root satisfies the inequality. That is, we need to check for the existence of such a number that satisfies the inequality in the first place. Our root, 6, is greater than $k=3$. Thus, such a number exists in our array. We then track the leftmost path that satisfies, the inequality, and we will therefore have the path:

![segment_tree_cascade_1](https://kroki.io/tikz/svg/eNqlVsGO2jAQvecrptIi7UqIQtitKlUcWnVvrVRVqnogHBxnElxMHNkOLI3y7x0HCAmbwHYXCeSMx8_vvRkcD-CXQVhptRJgFXCVblBbNxTrTGnLQokgRbryvCBSPF9jarlkxhRMW8Elll6QG8wYX7EECytWf9uRJ66k0vuYm5Ui1EzvCrNkGZpRgmqNVgveWjVn3IoNDq1IljajwKLING4Ebintx370mG6EVqkjVG2bCW5z7fjQRhLTxC6PqV-UjlAX49HEf8hsSVJCTERaHAWdIk2g-cADkLhBCZEwlqUcZ5PRR74ewgC-HiIQot0ipvtEQyswShBirdaQMU3g70fG7iTOikizbVmtxZjl0laZBm4VOb7VwlpCiZUGkyEXseCQqgjNnYOkjF31WINp5LR_InEIWuVphBHVTqeozRDcRrNQkpVDiIWUs0Sz3Tt_PASLTxY4kUKN0f6xTlSpnQVmzaQsh7XwSb2hESG1QXKy4oGvKzW_BXlLpBl3005ALLSxe4AayO8H8kdHqO-kUDOLLTSD1JTRGdz0AhyBEdbn6E9uLNnSxLJLpK_QR7iF5w3gp1K2MtcL3C_MD_7leLCP71ha23eYIBS-WkDheuTYOyzMJdNlQb0M9AlcehgXN2w-hoBHyhq4X9yUEATQSlizp9kHtybANKphPBcZwLd9HWjMl0JGULyOYy_Lbp5-k2cH03OuAPvoka8PtxJjCyYPrUa8qybfKOCChG4Rk7aIThnPhRylnMRM22JAxfBcXFteTfkiaYAm3fHipsUWwPH0T0DPiJ6onh86c7IuXZwlvdn_K2IaNZj01qCnCv8vzx1atYgFFeuxTpqPR6PJonqJEYVKNq16O6bfjVl6veW_4Be55Df-bi2TnDX3V_rzQsn7E64IvO8SWHqd4nqlkbBp47xrCDv189XTQ7s3f__x8UKDax7TDoP9foNfXdH7buEv2rD8RNeQaqp5m9mHTjeVf8SfF9I=)

Therefore, we have found the index of the leftmost number in $A$, such that it is greater than or equal to $k=3$.

It can be shown that we only need to visit at most $\log N$ nodes for any query.

#### Update

We now want to update the value in $A_1$ to subtract it by $k=3$. To do so, we need to update $A_1$ and all its parents. That is, all the nodes in blue in the diagram above needs to be updated.

To start off, we will update the node corresponding to $A_1$.

![segment_tree_update_1](https://kroki.io/tikz/svg/eNqlVsGK2zAQvfsrprCBFkKaOLulUHJoaW8tlELpIfZBlseOGlkykpzd1PjfO3I2jp11smUbSLBHo6f33kzGnsBPi7A1eivAaeBa7dA4fymKUhvHEokghdoGQZRqXhWoHJfM2poZJ7jEJogqiyXjW5Zj7cT2zyCyTneitIoVaOP6gWupzWHdZ0qRGGb2td2wEu0sR12gM4IPERh3YodTJ_KNKykQ16XBncB7Svt-uPqidsJo5cm1FErBXWU8NzpIosrd5pj6SZsUTT2fLcK70jUkK8FcqPoo7hTpA60nAYDEHUpIhXVMcVwtZu95MYUJfH6MQILuHlEdEi3twDRHyIwuoGSGwN_OrNtLXNWpYfdNuxczVknXZlp4rcn9eyOcI5RMG7AlcpEJDkqnaN94SMrYt7cdmEFO5-cSp2B0pVJMqY5GobFT8AetEklWTiETUq5yw_avwvkUHD444EQKDaaH2y5RK7eKbMGkbKad8EV3oBUJtUR-suKOF62aX4K8JdKM-2UvIBPGugNABxReBgpnR6hvpNAwhwM0i9Sg6Rnc8gocgRHWx_R3ZR3Z0sdyG6SvMEe4OAgm8ENr15obRP4X1o_-VfhoH98z1dn3uEAofBtD7Xvk2DssqSQzTU29DPSJfHqS1TdsPYeIp9pZuI1vGogiGCQU7GH1zu-JUKUdTOAjE_h6qANd842QKdQv43iR5TjPsM9zhOk5V4BD9Mg3hNcSMwe2SpxBfNMu_qeAKxLGRSyGIkZlPBVylHISsxyKAZ3BU3FDeR3lq6QB-nTn8c2ALYDnGZ6AnhA9UT0fOmuyTsVnSU_9b6dAa__HkmawdqcKfKL77Y92SpxV4RlJvUosLlZiWItl82KRfnR1rRRTyb50Sev5bLaI28caUWjF067_xwzHMZvgYhNc8YtcCnt_uoFJ3prbZ7r0SuEvJzwj8HZMYBOMirsojYQte1OvJ-zU1c_OEOOf_5eHyD8a3PFYjhgcXjb4xRW9HRf-Twc2H-hlpF3qv9McQqf3lb-NUB7Z)

We will then propagate upwards and compare the children of the parents to recalculate the maximum values, if applicable. In order, we have the first parent having a new maximum of 3, since its 2 children are 2 and 3.

![segment_tree_update_2](https://kroki.io/tikz/svg/eNqdVl2P2jgUfc-vuJUGqZUQhTCzWqniYWZ33rZSVWnbB5IHx7kJXpw4sh1m2Cj_vdcB8gGEaYsEcq6vj88598Z4Av8ahK1WWwFWAVf5DrV1Q5EVSlsWSQQp8q3nBbHiZYa55ZIZUzFtBZdYe0FpsGB8y1KsrNj-P4is450oTM4yNGH1ypVU-jDvMqWINNP7ymxYgWaWosrQasGHCIxbscOpFenGFhQIq0LjTuALpX05jJ7zndAqd-QaCoXgttSOG20kMU_t5pT6pHSMuprPFv5DYWuSFWEq8uokrov0gdYTD0DiDiXEwliWc1wtZn_ybAoT-PsYgQjtC2J-SDS0AuMUIdEqg4JpAv84M3YvcVXFmr3UzVpMWCltk2ngvSL3X7SwllASpcEUyEUiOOQqRvPBQVLGvnlswTRy2j-VOAWtyjzGmOqoc9RmCm6jVSTJyikkQspVqtn-nT-fgsVXC5xIocb48NgmqtyuApMxKetpK3zRbmhERC2RdlY88KxR812Qt0SacTftBCRCG3sAaIH8cSB_doL6TAo1szhAM0gNGp_BLW_AERhhPcb_lcaSLX0su0H6Cn2CCz1vAl-Vso25XuB-YX30r8SjfXzP8ta-4wSh8G0IleuRU--wqJRM1xX1MtAncOlRUt2x9RwCHitr4D68qyEIYJCQsdfVH25NgHncwnguMoF_DnWgMd8IGUP1exxHWV7n6fd5XmF6zhXgED3x9eG9xMSCKSOrET80k5cCmi5s-D8WdAYo20l4ouft16ZLBzJuCLkuZTGUMhSzbE04k3MS1ElaDiWBSuBS4lBkS_kmaYA-3Xl4N2AL4Hj6HdAF0Y7q-dGzJuvy8CzpsgrfhJJo3y3nx1r8xWK0T9RCXTWOKQ_zy3K8oa1XksVoSUaK8utq3Ul26qyQSvfc5qzns9kibP7kiEFjAi36FUj3Ul1i-tcxa2-0GW7YRSb5vVdw4JFz5v6Nbr3RAOMJbwi8vyaw9q6KG5VGwpa9M7AnrOvuN08U7W4D40fKTxrc8lheMdgfN_i3K3p_XfhPbVh_oqtJM9W_4RxC3e3lB1Z5Isc=)

Then, its parent has a new maximum of 4, since its children are 3 and 4.

![segment_tree_update_3](https://kroki.io/tikz/svg/eNqtlk9v2zgQxe_6FFMgBlrAcG05WSxQ-NBse-sCiwLbHiwdKGokc02RAkk58Qr67h3Jtv7YltMUDZBAIkeP7_eGYjSBfy3C1uitAKeBa7VD4-pLkeXaOBZJBCnU1vOCWPMiQ-W4ZNaWzDjBJVZeUFjMGd-yFEsntv8PRtbxTuRWsQxtWD5zLbU5zNeVUkSGmX1pNyxHO0tRZ-iM4EMFxp3Y4dSJdONyGgjL3OBO4BOV_XO4-qx2wmhVm2ss5IK7wtTeaCGJKnWbU-mjNjGacj5b-A-5qwgrwlSo8gTXjfSF1hMPQOIOJcTCOqY4rhazP3k2hQl8Oo5AhO4JUR0KLT2BcYqQGJ1BzgyJv59Zt5e4KmPDnqrmWUxYIV1TaeGtpvSfjHCOVBJtwObIRSI4KB2jfVdLUsW-uW3FDHJaP5U4BaMLFWNMfTQKjZ1CvdAqkhTlFBIh5So1bP_Gn0_B4bMDTqbQYHy4bQu1cqvAZkzKatqCL9oFrYhoS6RdFA88a2i-C8qWTDNeT9cAiTDWHQRaIX9cyJ-dpP4mQsMcDtQs0gaNz-SWN-RIjLQ-xv8V1lEsfS23QfoV5iQXet4EvmrtmnC9oP4L62N-BR7j43um2viOE6TCtyGU9R457R0WFZKZqqS9DPQT1OVRUt6x9RwCHmtn4T68qyAIYFCQsefVH_UzAaq4lfHqkQl8OfSBrvlGyBjKnsemiY3Fjzm9Qtp1Lh_pfvu1aXLP6ajX6279vtuh3_sj48AxwGH05NqHtxITB7aInEF810xeYnwTWqJ7s5wfYf5iMbpHirnDOZY8zM95bhBdZ1oMmYZUyzaNM64TWce2HLKBTuCSdUjbWr5pGqBvdx7eDdwC1D79TujCaGf1_CRaU3QqPCv63e14ga3XksVoS0aa8nra-mBrmDqG5TykHn5ui9fz2WwRNv_8yEqTBj39Gm16Cy8l_euSlTe6KW7ERmH5vXdykFXzMr6wa29shPGClq8-8C4B768BVt5VuFE0Alv2jsYeWLfLXzxiTP2RMH7G_GTArY_llYD98YB_uaP318F_asHqA32xNFP9D5_DUPdR8wOuZilo)

Lastly, the changes are propagated to the root, which has a new maximum of 4, since its children are 4 and 2.

![segment_tree_update_4](https://kroki.io/tikz/svg/eNqtVk1v2zgQvetXTIEYaAHDteUEWGDhQ7Pb2y6wKNDuwdKBokYya0oUyJGTrKD_viM51odjOW0RAzao4fDxvTcjmjP46hD21uwVkAFp8gNaaoYqK4wlEWkErfK95wWxkWWGOUktnKuEJSU11l5QOiyE3IsUK1L7_0aRbXxQhctFhi6sHqXRxh7nm0ytIivsU-V2okC3SNFkSFbJMYKQpA44J5XuqOBAWBUWDwofOO2f4-hzflDW5A25lkKhJJW24cYbacxT2p1S742N0VbLxcq_K6hmWRGmKq9O4vrIEGg78wA0HlBDrByJXOJmtfhNZnOYwZ_PEYiQHhDzY6LjFRinCIk1GRTCMvjHhaMnjZsqtuKhbtdiIkpNbaaD94bdf7CKiFESY8EVKFWiJOQmRvehgeSMp_axA7Moef9U4xysKfMYY66jzdG6OTQbbSLNVs4hUVpvUiue3vnLORA-EkgmhRbj42OXaHLaBC4TWtfzTviq29CpiFsi7a24k1mr5l_F3jJpIZvpRkCirKMjQAfkTwP5ixPU36zQCsIRmkNu0PgMbn0FjsEY61P8vXTEtgyxaIf8VfYEF3reDL4YQ625XtD8wrb1rzWode9Twe1pqDNwc8_P-y-tgTsehVA1nXLqIBGVWti64o4G_gTNkiipbsR2CYGMDTm4DW9qCAIYJWTicXPbrAkwjzsYr4nM4K9jNXgsd0rHUA2YflNGI71bL5_5_iFipHtdYs_4OeVuOaY8SfoybX9I-wLxc-oAx-iJvg_vNSYErozIIn5oJ99WzxVFlzWtxprGqtadG2e6Tsp6beuxNjAJvNQ6VttRvkoaYEh3Gd6M2AI0PP0e6AXRnur5wbRl6_LwLOmty_GKtkFJVpMlmSjKz6ttzrlWU69hvQy5hp-75O1ysViF7X8hU2nd4NVvh-1fxq69ye644h-75g9ezpFp7Vv5Svte6YjphE4on5Av9d1e0ld7F7VNKmNd68FZOdDVd_urR41t7g7TZ80P-tvxWF_w15_295cLentZ-A9tWP_OF5l2angfOob6u87_cHowCQ==)

Since we have reached the root, the update operation is now complete.

It can be shown that we will update at most $\log N$ nodes in the segment tree.

#### Completing the Solution

We can then perform the rest of the operations in order, from the rightmost memory block to the leftmost memory block. 

The checksum can be computed similar to part A, but we will need to store a prefix sum to determine the positions of memory blocks and free space respectively.

#### Implementation Details ($2N$ and $4N$ node segment trees)

Lastly, we will make a note on implementation.

To improve efficiency, we avoid storing a pointer to the left and right node in the segment tree. We therefore store the elements in an array, with their indices implicitly describing the structure of the segment tree.

There are two main ways to write a such a segment tree, the memory-efficient approach using $2N$ nodes, and the recursive approach using $4N$ nodes. 

In both approaches, the children of a node $x$ will be $2x$ and $2x + 1$. Conversely, the parent of a node will be $\lfloor \frac{x}{2} \rfloor$. However, they differ in where the nodes are stored.

* In the memory-efficient $2N$ node approach, the nodes are stored in order, that is, the root is stored at index $1$, and its children are stored at index $2$ and $3$. The children of index $2$ will be stored at indices $4$ and $5$, and that of index $3$ will be stored at indices $6$ and $7$. <br/> 
However, do note that not all nodes will necessarily have 2 children if the array size is not a power of 2, but the numbering will follow the order of the indices. <br/>
The array will therefore be stored at indices $s$ to $2s-1$, where $s$ is the size of the array. 

The $2N$ nodes memory-efficient segment tree cannot be used here, since it is not degenerate. For example,

<figure>
    <img src="https://codeforces.com/predownloaded/52/3e/523e873ebb079a80c9538c4205c8c243c66058ed.png"
         alt="non_degen_segtree">
    <figcaption><i>Source: <a href="https://codeforces.com/blog/entry/18051">Efficient and Easy Segment Trees, Codeforces</a></i></figcaption>
</figure>

We note that the right segment of the root actually contains the leftmost element in the array at index 13. Due to this, we cannot guarantee that the leftmost child in the segment tree corresponds to a left element in the array.

* In the $4N$ node approach, each level of the segment tree is padded to the a size that is a power of 2. <br/>
Therefore, the array will always be stored in the indices
$2^{\lceil\log_2s\rceil}$ to $2^{\lceil\log_2s\rceil} + s$, if the array is 1-indexed, where $s$ is the size of the array. <br/>
In such a case, the left child in the segment tree will always correspond to the an element that is left in the array.

Hence, we must use this $4N$ node implementation, which is commonly implemented using recursion, but can also be implemented using iteration (as with my solution).

### Code Complexity

**Time Complexity:** $O(N \log N)$

* $N$ is the number of characters in the string.

Each query and update takes $\log \frac{N}{2}$ time, since there are $\frac{N}{2}$ blocks of free space. We need to perform a query and update for every memory block in the string, of which there are $\frac{N}{2}$ such memory blocks.

**Additional Space Complexity:** $O(N)$

We need $4N$ nodes in the array to store the segment tree.

**Final answer:** 6493634986625.
