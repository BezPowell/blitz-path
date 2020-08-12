# blitz-path
![Rust](https://github.com/BezPowell/blitz-path/workflows/Rust/badge.svg)

Source for my experiments in implementing various pathfinding algorithms in rust. Currently in a very wip state. It relies on the [movingai-rust](https://github.com/THeK3nger/movingai-rust) crate for map implementation and testing / benchmarks.

## Usage
Provides a Route struct representing a path between two points and functions for each algorithm to calculate the shortest Route between two points.

## Testing
I have not included the .map and .scen files the tests use in case of any potential licensing issues, but they can be downloaded from the [movingai website](https://www.movingai.com/benchmarks/) and placed in the /tests/map folder. As a result of this the ci tests will always fail.
