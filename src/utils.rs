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
            let direction = direction(parent, node);
            let mut next = Coords2D::from((
                (node.0 as i32 + direction.0) as usize,
                (node.1 as i32 + direction.1) as usize,
            ));

            //Push intermidiate nodes if any
            while next != parent {
                path.push(next);
                next = Coords2D::from((
                    (next.0 as i32 + direction.0) as usize,
                    (next.1 as i32 + direction.1) as usize,
                ));
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
    //Calculate direction - needs cleaning
    let mut direction_x = current.0 as i32 - parent.0 as i32;
    let mut direction_y = current.1 as i32 - parent.1 as i32;
    if direction_x < 0 {
        direction_x = -1;
    } else if direction_x > 0 {
        direction_x = 1;
    }
    if direction_y < 0 {
        direction_y = -1;
    } else if direction_y > 0 {
        direction_y = 1;
    }

    (direction_x, direction_y)
}
