use movingai::Coords2D;
use std::cmp::Ordering;

use super::utils::distance;

#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub f: f64,
    pub g: f64,
    pub h: f64,
    pub position: Coords2D,
    pub parent: Coords2D,
}

impl Node {
    pub fn new(g: f64, h: f64, position: Coords2D, parent: Coords2D) -> Node {
        Node {
            f: g + h,
            g,
            h,
            position,
            parent,
        }
    }

    pub fn from_parent(parent: &Node, position: Coords2D, goal: Coords2D) -> Node {
        //Calculate distances
        let distance_to_goal = distance(position, goal);
        let distance_from_parent = distance(parent.position, position);
        let total_distance = parent.g + distance_from_parent;

        //Build node from distances
        Node {
            f: total_distance + distance_to_goal,
            g: total_distance,
            h: distance_to_goal,
            position: position,
            parent: parent.position,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        // This is reversed on purpose to make the max-heap into min-heap.
        other.f.partial_cmp(&self.f).unwrap()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.position == other.position
    }
}

impl Eq for Node {
    // add code here
}
