use std::collections::BinaryHeap;

use movingai::Coords2D;
use movingai::Map2D;

use crate::node::Node;
use crate::utils::{direction, distance, rewind};
use crate::Route;

#[derive(Copy, Clone)]
enum Direction {
    Vertical(i32),
    Horizontal(i32),
    Diagonal(i32, i32),
}

///Creates a new route using the JPS algorithm.
///Returns a Route struct containing the distance to the goal and number of steps needed to get there.
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let map = movingai::parser::parse_map_file(Path::new("./tests/map/maze512-32-9.map")).expect("Could not load map.");
/// let scenes = movingai::parser::parse_scen_file(Path::new("./tests/map/maze512-32-9.map.scen")).expect("Could not load scenario.");
/// let scene = &scenes[0];
///
/// let path = blitz_path::jps_path(&map, scene.start_pos, scene.goal_pos);
///
/// // using as f32 as scene.optimal_length is stored as f64,
/// // but only seems to have precision to f32
/// if let Some(path) = path {
///     assert_eq!(scene.optimal_length as f32, path.distance() as f32);  
/// }
/// ```

#[inline]
pub fn jps_path<U, T: Map2D<U>>(map: &T, start: Coords2D, goal: Coords2D) -> Option<Route> {
    if start == goal {
        return Some(Route::from((0.0, vec![])));
    }

    //Push start node to open list
    let start_node = Node::new(0.0, distance(start, goal), start, start);

    //Add start's neighbours to open list - modified as seems to be error in neighbours function
    let prev_x = start_node.position.0 - 1;
    let next_x = start_node.position.0 + 1;
    let prev_y = start_node.position.1 - 1;
    let next_y = start_node.position.1 + 1;

    //Initialize open and closed lists
    let capacity = (1 + next_x - prev_x) * (1 + next_y - prev_y);
    let mut open = BinaryHeap::with_capacity(capacity);
    let mut closed = Vec::with_capacity(capacity);

    for x in prev_x..=next_x {
        for y in prev_y..=next_y {
            open.push(Node::from_parent(&start_node, (x, y), goal));
        }
    }

    closed.push(start_node);

    //Examine the nodes
    while let Some(node_current) = open.pop() {
        //If this is the target node return the distance to get there
        if node_current.position == goal {
            //Push all remaining to closed
            closed.append(&mut open.into_vec());

            //Unwind
            let path = rewind(&node_current, &closed);
            let route = Route::from((node_current.g, path));
            return Some(route);
        }

        //Check if node is on closed list and continue if is
        if closed.contains(&node_current) {
            continue;
        }

        //Calculate direction
        let direction = direction(node_current.position, node_current.parent);

        if let Some(nodes) = check_jump(&node_current, map, direction, goal) {
            for node in nodes {
                open.push(node);
            }
        }

        //Push current node to closed list
        closed.push(node_current);
    }

    None
}

#[inline]
fn check_jump<U, T: Map2D<U>>(
    parent: &Node,
    map: &T,
    (dx, dy): (i32, i32),
    goal: Coords2D,
) -> Option<Vec<Node>> {
    if dx != 0 {
        if dy != 0 {
            expand(map, &parent, Direction::Diagonal(dx, dy), goal)
        } else {
            expand(map, &parent, Direction::Horizontal(dx), goal)
        }
    } else if dy != 0 {
        expand(map, &parent, Direction::Vertical(dy), goal)
    } else {
        None
    }
}

#[inline]
fn forced_horizontal<U, T: Map2D<U>>(
    nodes: &mut Vec<Node>,
    map: &T,
    check_node: &Node,
    direction: i32,
    goal: Coords2D,
) {
    let (check_x, check_y) = check_node.position;
    let next_x = (check_x as i32 + direction) as usize;
    let up_y = (check_y as i32 - 1) as usize;
    let down_y = (check_y as i32 + 1) as usize;

    //Check if blocked up
    if !map.is_traversable((check_x, up_y)) && map.is_traversable((next_x, up_y)) {
        nodes.push(Node::from_parent(&check_node, (next_x, up_y), goal));
    }

    //Check if blocked down
    if !map.is_traversable((check_x, down_y)) && map.is_traversable((next_x, down_y)) {
        nodes.push(Node::from_parent(&check_node, (next_x, down_y), goal));
    }
}

#[inline]
fn forced_vertical<U, T: Map2D<U>>(
    nodes: &mut Vec<Node>,
    map: &T,
    check_node: &Node,
    direction: i32,
    goal: Coords2D,
) {
    let (check_x, check_y) = check_node.position;
    let left_x = (check_x as i32 - 1) as usize;
    let right_x = (check_x as i32 + 1) as usize;
    let next_y = (check_y as i32 + direction) as usize;

    //Check if blocked left
    if !map.is_traversable((left_x, check_y)) && map.is_traversable((left_x, next_y)) {
        nodes.push(Node::from_parent(&check_node, (left_x, next_y), goal));
    }

    //Check if blocked right
    if !map.is_traversable((right_x, check_y)) && map.is_traversable((right_x, next_y)) {
        nodes.push(Node::from_parent(&check_node, (right_x, next_y), goal));
    }
}

#[inline]
fn expand<U, T: Map2D<U>>(
    map: &T,
    start_node: &Node,
    direction: Direction,
    goal: Coords2D,
) -> Option<Vec<Node>> {
    let mut current = *start_node;
    let mut nodes = Vec::new();
    loop {
        //Check if goal
        if current.position == goal {
            nodes.push(current);

            return Some(nodes);
        }

        //Check blocked
        if !map.is_traversable(current.position) {
            return None;
        }

        //Otherwise Expand depending on direction
        let dir = match direction {
            Direction::Vertical(vert) => {
                //Check for forced neighbours
                forced_vertical(&mut nodes, map, &current, vert, goal);

                (0, vert)
            }
            Direction::Horizontal(hor) => {
                //Check for forced neighbours
                forced_horizontal(&mut nodes, map, &current, hor, goal);

                (hor, 0)
            }
            Direction::Diagonal(hor, vert) => {
                //Expand horizontally
                if let Some(mut hor_nodes) = expand(map, &current, Direction::Horizontal(hor), goal)
                {
                    nodes.append(&mut hor_nodes);
                }
                //Expand vertically
                if let Some(mut vert_nodes) = expand(map, &current, Direction::Vertical(vert), goal)
                {
                    nodes.append(&mut vert_nodes);
                }

                (hor, vert)
            }
        };

        let next_x = (current.position.0 as i32 + dir.0) as usize;
        let next_y = (current.position.1 as i32 + dir.1) as usize;
        let next_position = (next_x, next_y);

        //If forced neighbours found return them along with this node and next on to continue checking in this direction
        if !nodes.is_empty() {
            let next_node = Node::from_parent(&current, next_position, goal);
            nodes.push(current);
            nodes.push(next_node);

            return Some(nodes);
        }

        //Else move onto next tile
        current = Node::from_parent(start_node, next_position, goal);
    }
}
