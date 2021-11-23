# AOC 2021

This is supposed to be a commandline tool for the [adventofcode](https://adventofcode.com).

## Library

The [library](./libaoc) is essentially a helper to build the commandline tool. It automatically fetches the inputs of each day when they are needed.

Using this library you can either build your own commandline tool, or you can modify the [existing one](./aoc) and add your own solutions.

## Binary

The binary [aoc](./aoc) is the main part and in it you will find my solutions for each day as an example. It uses the library and provides a commandline tool to run each day.