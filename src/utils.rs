use movingai::Coords2D;

use super::node::Node;

pub fn distance(a: Coords2D, b: Coords2D) -> f64 {
    let (x, y) = (a.0 as f64, a.1 as f64);
    let (p, q) = (b.0 as f64, b.1 as f64);
    ((x - p) * (x - p) + (y - q) * (y - q)).sqrt()
}

pub fn unwind(start: &Node, closed: &Vec<Node>) -> Vec<Coords2D> {
    let mut path = Vec::new();
    path.push(start.position);
    let mut parent = start.parent;
    let mut node = start.position;

    while parent != node {
        if let Some(step) = closed.iter().find(|x| x.position == parent) {
            parent = step.parent;
            node = step.position;
            path.push(node);
        }
    }

    path
}
