use std::collections::BinaryHeap;

use movingai::Coords2D;
use movingai::Map2D;

use crate::node::Node;
use crate::utils::{distance, rewind};
use crate::Route;

///Creates a new route using the A* algorithm.
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
/// let path = blitz_path::a_star_path(&map, scene.start_pos, scene.goal_pos);
///
/// // using as f32 as scene.optimal_length is stored as f64,
/// // but only seems to have precision to f32
/// if let Some(path) = path {
///     assert_eq!(scene.optimal_length as f32, path.distance() as f32);  
/// }
/// ```

pub fn a_star_path<U, T: Map2D<U>>(map: &T, start: Coords2D, goal: Coords2D) -> Option<Route> {
    if start == goal {
        return Some(Route::from((0.0, vec![])));
    }

    //Initialize open and closed lists
    let capacity = (1 + start.0 + 1 - start.0 - 1) * (1 + start.1 + 1 - start.1 - 1);
    let mut open = BinaryHeap::with_capacity(capacity);
    let mut closed = Vec::<Node>::with_capacity(capacity);

    //Push start node to open list
    open.push(Node {
        f: distance(start, goal),
        g: 0.0,
        h: distance(start, goal),
        position: start,
        parent: start,
    });

    //Examine the nodes
    while let Some(node_current) = open.pop() {
        //If this is the target node return the distance to get there
        if node_current.position == goal {
            let path = rewind(&node_current, &closed);
            let route = Route::from((node_current.g, path));
            return Some(route);
        }

        //Setup successor nodes
        for successor in map.neighbors(node_current.position) {
            //Calculate distances
            let distance_to_goal = distance(successor, goal);
            let distance_from_parent = distance(node_current.position, successor);
            let total_distance = node_current.g + distance_from_parent;

            //Check if node is on closed list
            if let Some(closed) = closed.iter().find(|x| x.position == successor) {
                //and skip if existing score is better
                if closed.f < total_distance + distance_to_goal {
                    continue;
                }
            }

            //See if node is on open list
            if let Some(old_open) = open.iter().find(|x| x.position == successor) {
                //Continue if we already have a better result
                if old_open.g < total_distance {
                    continue;
                }
            }

            //If sill best distance build node and push to open list
            //Build node from distances
            let node_successor = Node {
                f: total_distance + distance_to_goal,
                g: total_distance,
                h: distance_to_goal,
                position: successor,
                parent: node_current.position,
            };

            open.push(node_successor);
        }

        //Push current node to closed list and remove existing entry if duplicate
        if let Some(index) = closed
            .iter()
            .position(|x| x.position == node_current.position)
        {
            closed.remove(index);
        }
        closed.push(node_current);
    }

    None
}
