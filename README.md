# zobrist-gen [<img src="https://travis-ci.org/kuking/zobrist-gen.svg?branch=master">](https://travis-ci.org/kuking/zobrist-gen)
Generates sets of numbers satisfying a Hamming distance range for building good Zobrist hashes. For more information you can read the Wikipedia page on [Zobrish hashes](https://en.wikipedia.org/wiki/Zobrist_hashing) and the page on [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance).

In a nutshell, Zobrist hashes are built by XORing sequences of numbers; each
number represent a portion of the state, by XORing all the parts together we
obtain an unique-ish number, a hash for the state built by the sum of all the sub-states.

Because `a xor a = 0`, we want the numbers utilised as keys to be _as different
as possible_ so we don't risk generating hashes with many 0 bits (or 1s), but
**not too different** as we would run out of possible numbers (bitwise difference)

i.e. It is impossible to generate 127 numbers of 8 bits with a minimum Hamming
distance of 8, there are only 8 possible numbers: `00000001, 00000010, ...
1000000.`).

## Usage
```
$ zobrist-gen --help
zobrist-gen 0.1 by Eduardo ES Riccardi (https://github.com/kuking/zorbist-gen)
Usage: zobrist-gen [options]
       zobrist-gen --help

Options: -b <nbits>  Bits to generate in the output, default 64
         -l <min>    Minimum number of different bits between all generated numbers numbers
                     (Hamming distance) - default 20
         -u <max>    Maximum Hamming distance between values - default 45
         -q <qty>    Quantity of numbers to generate - default 1083

         i.e. zobrist-gen -b 32 -l 10 -u 24 -q 1083
          or: zobrist-gen -b 128 -l 40 -u 90 -q 1083
          or: zobrist-gen -b 16 -l 5 -u 12 -q 243
```

### Output
````
$ zobrist-gen
zobrist-gen 0.1 by Eduardo ES Riccardi (https://github.com/kuking/zorbist-gen)
Generating 64 bits numbers with a Hamming distance between 20 and 45.
    0] 0011001100001101001101000111101001111010010010101001101010011010 [0.0%
    1] 0100010111101111100011011101101010001111100100010001011000001000 [0.1%
    2] 0101110000101000010111001110011000101101101101100010100010100111 [0.2%
    3] 0100010000100101001100000110100101001101011100110111000010111001 [0.3%
    4] 0110111100110111010010110010001000110001000110010100110011100100 [0.4%
    5] 0110001000000101110011111010111010001110100111100111100110001101 [0.5%
    6] 0001001111101000111111001111101000111100001110110101011101010001 [0.6%
    7] 1001000101000100110010110101111110110100110100110011101111000010 [0.6%
    8] 0100010101101011001010100100010010010000100000011001000001010011 [0.7%
[truncated]
 1082] 1011000110001100100111001100110011110111111101111110110110101101 [99.9%

>-+
  | Friendly for your favourite compiler:
  +--------------------------------------->

0x330d347a7a4a9a9a, 0x45ef8dda8f911608, 0x5c285ce62db628a7, 0x442530694d7370b9,
0x6f374b2231194ce4, 0x6205cfae8e9e798d, 0x13e8fcfa3c3b5751, 0x9144cb5fb4d33bc2,
0x456b2a4490819053, 0x9f0fd493e6650017, 0x6bb32b911ef06cb3, 0x4fc0e21a2dcb7735,
0xac86770c55fd490a, 0x0c3a445102c38a43, 0x7978f28ae9e4b56a, 0x61e43e6544442ce6,
[truncated]
0xedeab3006e6183da, 0x9a5de74b6b76b211, 0x10fc3290d21e8155, 0x2fb6caccb7ea0ec4,
0xed4bc17d5fc1407c, 0xb2f3db9b32d3f992, 0xb18c9cccf7f7edad,

Sanity check ... Checking if a random combination of the numbers will XOR into 0
(!) This should not be a replacement to a well selected pair of '-l' & '-u' params

I will check 100K times; you might want to ctrl-C (a tick is 1k)
/////////////////////////

Sanity passed; Generated numbers might be good enough for a Zobrist hash.
````
