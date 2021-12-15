# AOC 2021

## Progress

| Day | Problem | Part 1 | Part 2 |
| :-: |	:------- | :----: | :----: |
| [1](aoc/src/days/one.rs) | Sonar Sweep | ★ | ★ |
| [2](aoc/src/days/two.rs) | Dive! | ★ | ★ |
| [3](aoc/src/days/three.rs) | Binary Diagnostic | ★ | ★ |
| [4](aoc/src/days/four.rs) | Giant Squid | ★ | ★ |
| [5](aoc/src/days/five.rs) | Hydrothermal Venture | ★ | ★ |
| [6](aoc/src/days/six.rs) | Lanternfish | ★ | ★ |
| [7](aoc/src/days/seven.rs) | The Treachery of Whales | ★ | ★ |
| [8](aoc/src/days/eight.rs) | Seven Segment Search | ★ | ★ |
| [9](aoc/src/days/nine.rs) | Smoke Basin | ★ | ★ |
| [10](aoc/src/days/ten.rs) | Syntax Scoring | ★ | ★ |
| [11](aoc/src/days/eleven.rs) | Dumbo Octopus | ★ | ★ |
| [12](aoc/src/days/twelve.rs) | Passage Pathing | ★ | ★ |
| [13](aoc/src/days/thirteem.rs) | Transparent Origami | ★ | ★ |
| [14](aoc/src/days/fourteen.rs) | Extended Polymerization | ★ |  |
<!--
| [15](aoc/src/days/fifteen.rs) |  |  |  |
| [16](aoc/src/days/sixteen.rs) |  |  |  |
| [17](aoc/src/days/seventeen.rs) |  |  |  |
| [18](aoc/src/days/eighteen.rs) |  |  |  |
| [19](aoc/src/days/nineteeen.rs) |  |  |  |
| [20](aoc/src/days/twenty.rs) |  |  |  |
| [21](aoc/src/days/twentyone.rs) |  |  |  |
| [22](aoc/src/days/twentytwo.rs) |  |  |  |
| [23](aoc/src/days/twentythree.rs) |  |  |  |
| [24](aoc/src/days/twentyfour.rs) |  |  |  |
| [25](aoc/src/days/twentyfive.rs) |  |  |  |
-->

## Information about this repository

This is supposed to be a commandline tool for the [adventofcode](https://adventofcode.com/2021).

### Library

The [library](./libaoc) is essentially a helper to build the commandline tool. It automatically fetches the inputs of each day when they are needed.

Using this library, you can either build your own commandline tool, or you can modify the [existing one](./aoc) and add your own solutions.

### Binary

The binary [aoc](./aoc) is the main part and in it you will find my solutions for each day as an example. It uses the library and provides a commandline tool to run each day.

## Other
- [Haskell solutions 2020](https://github.com/wiebecommajonas/aoc-2020)