//! # blitz-path
//!
//! `blitz-path` contains (hopefully) lightning-quick implementations of various pathfinding algorithms. Currently in a very wip state. It relies on the [movingai-rust](https://github.com/THeK3nger/movingai-rust) crate for map implementation and testing / benchmarks.
//!
//! It currently provides implementations of the A* and JPS pathfinding algorithms.

mod astar;
mod jps;
mod node;
mod route;
mod utils;

pub use astar::a_star_path;
pub use jps::jps_path;
pub use route::Route;
