# zobrist-gen [<img src="https://travis-ci.org/kuking/zobrist-gen.svg?branch=master">](https://travis-ci.org/kuking/zobrist-gen)
Generates sets of numbers satisfying a Hamming distance range for building good Zobrist hashes. For more information you can read the Wikipedia page on [Zobrish hashes](https://en.wikipedia.org/wiki/Zobrist_hashing) and the page on [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance).

In a nutshell, Zobrist hashes are built by XORing sequences of numbers; each
number represent a portion of the state, by XORing all the parts together we
obtain an unique-ish number representing the state; hopefully without much
collisions.

Because `a ⊕ a = 0 ∀ a`, we want the numbers utilised as keys to be _as different
as possible_ so we don't risk generating hashes with many 0 bits (or 1s)
and by doing so building hashes prone to having a high rate of collisions;
also, the numbers _should not be too different_ as we might not be able to
generate the desired set of numbers (bitwise difference).

There is a balance to be found here. i.e. It is impossible to generate a set of
more than 8 numbers of 8 bits with a maximum Hamming distance of 1:
`00000001, 00000010, ... 1000000.` (a set with absolute no collision), and even
less for minimum Hamming distance of 7 (only 2).

At the end of the process, there is a sanity check that will try to find a
sequences of `n` numbers that will generate a xor value of 0. By finding this sequence, we have found the sequence of `n-1` elements that will produce the
`n` element; therefore rendering its result to `0` (because `a ⊕ a = 0 ∀ a`).
If the sets of numbers have a finite (and relatively short) sequence producing
a cycle, it is not a good set of numbers and we want to avoid them. Choosing
a good Hamming range should be enough.

## Usage
```
$ zobrist-gen --help
zobrist-gen 0.1 by Eduardo ES Riccardi (https://github.com/kuking/zorbist-gen)
Usage: zobrist-gen [options]
       zobrist-gen --help

Options: -b <nbits>  Bits to generate in the output - default 64
         -l <min>    Minimum number of different bits between all generated numbers numbers
                     (Hamming distance) - default 20
         -u <max>    Maximum Hamming distance between values - default 45
         -q <qty>    Quantity of numbers to generate - default 722

         i.e. zobrist-gen -b 32 -l 10 -u 24 -q 722
          or: zobrist-gen -b 128 -l 40 -u 90 -q 722
          or: zobrist-gen -b 16 -l 5 -u 12 -q 162
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
  719] 1110100100001111100111010010100100000100001010011111100110000111 [99.6%
  720] 0010110101011000001100000000000100110001010011011001000001011010 [99.7%
  721] 0010101011101110011010110010010101101110010010010000110111010010 [99.9%

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
