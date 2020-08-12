# blitz-path
![Rust](https://github.com/BezPowell/blitz-path/workflows/Rust/badge.svg)

Source for my experiments in implementing various pathfinding algorithms in rust. Currently in a very wip state. It relies on the [movingai-rust](https://github.com/THeK3nger/movingai-rust) crate for map implementation and testing / benchmarks.

It currently provides implementations of the A* and JPS pathfinding algorithms.

## Usage
Provides a Route struct representing a path between two points and functions for each algorithm to calculate the shortest Route between two points.

## Testing
The .map and .scen files used for integration tests were provided by the [Moving AI Lab](https://www.movingai.com/benchmarks/) and are distributed with permission.
