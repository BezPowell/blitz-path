use movingai::Coords2D;

use crate::node::Node;

pub fn distance(a: Coords2D, b: Coords2D) -> f64 {
    let (x, y) = (a.0 as f64, a.1 as f64);
    let (p, q) = (b.0 as f64, b.1 as f64);
    ((x - p) * (x - p) + (y - q) * (y - q)).sqrt()
}

//Helper function to recreate path once goal is located
pub fn rewind(start: &Node, closed: &Vec<Node>) -> Vec<Coords2D> {
    let mut path = Vec::with_capacity(closed.len().pow(2) + 1);

    path.push(start.position);

    let mut parent = start.parent;
    let mut node = start.position;

    while parent != node {
        if let Some(step) = closed.iter().find(|x| x.position == parent) {
            let direction = direction(parent, node);
            let mut next = shift(node, direction);

            //Push intermidiate nodes if any
            while next != parent {
                path.push(next);

                next = shift(next, direction);
            }

            //Push actual steps
            parent = step.parent;
            node = step.position;
            path.push(node);
        }
    }

    path
}

pub fn direction(current: Coords2D, parent: Coords2D) -> (i32, i32) {
    (
        current.0.cmp(&parent.0) as i32,
        current.1.cmp(&parent.1) as i32,
    )
}

fn shift(node: Coords2D, direction: (i32, i32)) -> Coords2D {
    (
        (node.0 as i32 + direction.0) as usize,
        (node.1 as i32 + direction.1) as usize,
    )
}
