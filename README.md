# three-style

[![Build/test](https://github.com/luckasRanarison/three-style/actions/workflows/ci.yml/badge.svg)](https://github.com/luckasRanarison/three-style/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/three-style)](https://crates.io/crates/three-style)

three-style is a program that searches 3x3 commutators used in 3-style, an advanced method for solving the Rubik's cube blindfolded by swapping 3 pieces at a time without affecting the rest of the cube.

## Contents

- [Installation](#installation)
- [Usage](#usage)
- [Concept](#concept)
- [References](#concept)
- [Contributing](#contributing)

## Installation

You can use three-style by downloading the prebuilt binary for your system from the [releases](https://github.com/luckasRanarison/three-style/releases) or by installing it using cargo:

```bash
cargo install three-style
```

## Usage

three-style uses layers for describing pieces or more precisely sticker targets. Example: `UF` (edge), `UBL` (corner). Currently, it only exposes one main command `search` and is used in the following way:

```bash
# corners
three-style search --gen RUD --corners UFR UBL RFD --depth 4

# edges
three-style search --gen RUE --edges UF UB LF --depth 5

# shorter versions
three-style search -g RUD -c UFR UBL RFD -d 4
three-style search -g RUE -e UF UB LF -d 5
```

> [!NOTE]
> Depth is relative to the length of the commutator in its notation form.

## Concept

A commutator is an algorithm of the form: `A B A' B'` which allows us to swap 3 pieces at once without affecting the rest of the cube. It's commonly described in the following notation: `[A, B]`.

It consists of two basic interchangeable parts:

- An **interchange** is a single move that swaps two pieces without affecting the third one
- An **insertion** are three moves that insert the third piece into one of the two pieces spot without affecting the other one.

But not all cases can be solved using pure commutators, some cases require using setup moves. A **setup move** is a sequence of moves that turn the case into a case that can be solved using pure commutators. Commutators that use setup moves are of the form `S A B A' B' S'` and are more commonly written as `[S: [A, B]]`

> [!NOTE]
> Edges have a special case called **4 movers** which don't use the normal 3 moves insertion. Example: `[M', U2]`

Here are the steps to find any commutator:

```
           *------------------*
           | Choose a moveset |
           *------------------*
                    |
                    v
          *--------------------*  no   *------------------*
          | Interchange exists | ----> | Find setup moves |
          *--------------------* <---- *------------------*
                    | yes                        ^
                    v                            |
           *------------------*        no        |
           | Insertion exists | ------------------
           *------------------*
                    | yes
                    v
    *--------------------------------*
    | Interchange or insertion first |
    *--------------------------------*
                    |
                    v
                *------*
                | Done |
                *------*
```

The program basically does an iterative DFS and follows these steps to find commutators. The rules for the interchange and the insertion are used for prunning and search is decently fast.

## References

- [3-style tutorial by Timothy Goh](https://youtu.be/Bq9oz1k5wP4?si=fC3Xi_7j0ehMaepG)

- [Cube explorer](http://kociemba.org/cube.htm)

## Contributing

Bug reports, Pull requests and feature requests are all welcome!
