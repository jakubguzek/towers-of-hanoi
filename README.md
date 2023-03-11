# Tower of Hanoi

[![Build status](https://github.com/jakubguzek/towers-of-hanoi/actions/workflows/rust.yml/badge.svg)](https://github.com/jakubguzek/towers-of-hanoi/actions)

Tower of Hanoi is a famous puzzle in which one is presented with three rods and some number of disks of various diameters. Disks can be slid onto the rods, but with the limitation, that at no point larger disk can be on top of a smaller one.

To be precise, the following rules must be obeyed in this puzzle:
1. Only one disk may be moved at the time
2. Each move consists of taking the upper disk from one of the stacks and placing it on top of another stack or on an empty rod.
3. No disk may be placed on top of a disk that is smaller than it.

[!["Tower of Hanoi (from wikipedia)](Tower_of_Hanoi.jpeg "Tower of Hanoi model (from wikipedia)")](https://en.wikipedia.org/wiki/Tower_of_Hanoi)

It's a classic example of a discrete mathematical problem which can be solved by using recursion, although iterative algorithms also exist.

In this repository you can look at my implementation of this recursive algorithm in Rust.

Licensed under MIT license.

## Installation

If you are interested in running this code you will need to clone this repository and then build the crate using `setup.sh` script. To do so you will need [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

To build hanoi:
```
$ git clone https://github.com/jakubguzek/towers-of-hanoi
$ cd ./towers-of-hanoi
$ ./setup.sh
```
Then you can run the binary by entering
```
$ ./hanoi NUMBER_OF_DISKS
```
into your command-line while you are in the project's directory. To run the command from anywhere you will need to set you $PATH environmental variable accordingly.

For full help enter `./hanoi -h` or `./hanoi --help` into your command-line.

## Running tests

I wrote some unit tests for this project to have fun with it. To run them use:
```
$ cargo test --all
```
