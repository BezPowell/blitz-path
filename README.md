# blitz-path
![Rust](https://github.com/BezPowell/blitz-path/workflows/Rust/badge.svg)

Source for my experiments in implementing various pathfinding algorithms in rust. Currently in a very wip state. It relies on the [movingai-rust](https://github.com/THeK3nger/movingai-rust) crate for map implementation and testing / benchmarks.

## Usage
Import the library as usual and bring the blitz_path::Route struct into scope. This currently provides 2 functions to make a new path, using either the A* or JPS algorithms.

## Testing
I have not included the .map and .scen files the tests use in case of any potential licensing issues, but they can be downloaded from the [movingai website](https://www.movingai.com/benchmarks/) and placed in the /tests/map folder. 
