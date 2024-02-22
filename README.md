# three-style

three-style is a program for searching 3x3 commutators for the 3-style method which is an advanced method for solving the Rubik's cube blindfolded by swapping 3 pieces at a time without affecting the rest of the cube.

## Contents

- [Installation](#installation)
- [Usage](#usage)
- [Concept](#concept)

## Installation

You can use three-style by downloading the prebuilt binaries for your system from the [releases](https://github.com/luckasRanarison/three-style/releases) or by installing it using cargo:

```bash
cargo install three-style
```

## Usage

three-style uses layers for describing pieces or more precisely sticker targets. Example: `UF` (edge), `UBL` (corner). Currently, it only exposes one main command `search` and used in the following way:

```bash
three-style search --gen RUD --corners UFR UBL RFD --depth 4
three-style search -g RUD -c UFR UBL RFD -d 4
three-style search --gen RUE --edges UF UB LF --depth 4
three-style search -g RUD -e UF UB LF -d 4
```

## Concept

A commutator is an algorithm of the form: `A B A' B'` which allows us to swap 3 pieces at once without affecting the rest of the cube. It's commonly described in the following notation: `[A, B]` in blindsolving.

It consists of two basic parts:

- An **interchanhe** is a single move that swaps two pieces without affecting the third one
- An **insertion** are three moves that insert the third piece into one of the two pieces spot without affecting the other one.

But not all cases can be solved using pure commutators, some cases require using setup moves. A **setup move** is a sequence of moves that turn the case into a case that can be solved using pure commutators. Commutators that use setup moves are of the form `S A B A' B' S'` and are more commonly written as `[S: [A, B]]`

**Note**: Edges has a special case called **4 movers** which don't use the normal 3 moves insertion. Example: `[M', U2]`

## References

- [3 style tutorial by Timothy Goh](https://youtu.be/Bq9oz1k5wP4?si=fC3Xi_7j0ehMaepG)
