| Graph      | Partitioning Parts | Posible partitioning configurations |
| ---------- | ------------------ | ----------------------------------- |
| AlexNet    | 8                  | 45                                  |
| GoogleNet  | 13                 | 105                                 |
| MobileNet  | 28                 | 435                                 |
| ResNet50   | 18                 | 190                                 |
| SqueezeNet | 19                 | 210                                 |

Possible partitioning configurations = 1 + 2 + ... + (PP+1) = (PP+1)((PP+1)+1)/2

| Big Cluster Frequencies |
| ----------------------- |
| 500000                  |
| 667000                  |
| 1000000                 |
| 1200000                 |
| 1398000                 |
| 1512000                 |
| 1608000                 |
| 1704000                 |
| 1800000                 |
| 1908000                 |
| 2016000                 |
| 2100000                 |
| 2208000                 |
| Count: 13               |

| Small Cluster Frequencies |
| ------------------------- |
| 500000                    |
| 667000                    |
| 1000000                   |
| 1200000                   |
| 1398000                   |
| 1512000                   |
| 1704000                   |
| 1800000                   |
| Count: 8                  |

| Orders   |
| -------- |
| G-B-L    |
| G-L-B    |
| B-G-L    |
| B-L-G    |
| L-G-B    |
| L-B-G    |
| Count: 6 |

| Graph      | Configs |
| ---------- | ------- |
| AlexNet    | 28080   |
| GoogleNet  | 65520   |
| MobileNet  | 271440  |
| ResNet50   | 118560  |
| SqueezeNet | 131040  |

## Constraints

GPU pipeline stage duration can only be changed by partitioning.
CPU frequency must be changed to be as close as possible to GPU time while.
Is relationship between frequency and processing time linear?
Input time should be constant for each transition (G->B for PP 1 or B->L for PP 12)

## Strategy 1

Configurations for alexnet = 45 \* 13 \* 8 \* 6 = 28080  
If we keep order G-B-L = 45 \* 13 \* 8 = 4680  
If we use binary search to find optimal frequency = 45 \* log2(13) \* log2(8) = 499.559361949  
Run them all and choose best power efficient one

## Strategy 2

Brute force random strategies, run them all and choose best power efficient one which is valid.

## Strategy 3

Calculate statistics per pivot point, use those statistics to get the best possible result.  
Big CPU time per pivot point, PP calculations (PP = amount of partitioning parts)  
Little CPU time per pivot point, PP calculations  
GPU time per pivot point, PP calculations  
Total calculations = 3\*PP  
Then choose optimal configuration with this knowledge.  
CPU speed should be linear with frequency, so no need to calculate for each individual freq.

We know the input time, so we only need to calculate this once per PP per GP/Little/Big.
