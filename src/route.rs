use std::collections::BinaryHeap;

use movingai::Coords2D;
use movingai::Map2D;
use movingai::MovingAiMap;

use super::jps::check_jump;
use super::node::Node;
use super::utils::{distance, unwind};

pub struct Route {
    distance: f64,
    steps: Vec<Coords2D>,
}

impl Route {
    pub fn steps(&self) -> Vec<Coords2D> {
        self.steps.clone()
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    ///Creates a new route using the A* algorithm.
    ///Returns a Route struct containing the distance to the goal and number of steps needed to get there.
    pub fn a_star_path(map: &MovingAiMap, start: Coords2D, goal: Coords2D) -> Option<Route> {
        //Initialize open and closed lists
        let mut open = BinaryHeap::new();
        let mut closed = Vec::<Node>::new();

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
                let path = unwind(&node_current, &closed);
                return Some(Route {
                    distance: node_current.g,
                    steps: path,
                });
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

    ///Creates a new route using the JPS algorithm.
    ///Returns a Route struct containing the distance to the goal and number of steps needed to get there.
    pub fn jps_path(map: &MovingAiMap, start: Coords2D, goal: Coords2D) -> Option<Route> {
        //Initialize open and closed lists
        let mut open = BinaryHeap::new();
        let mut closed = Vec::<Node>::new();

        //Push start node to open list
        let start_node = Node::new(0.0, distance(start, goal), start, start);
        if start == goal {
            open.push(start_node);
        } else {
            //Add start's neighbours to open list
            for neighbour in map.neighbors(start) {
                let node = Node::from_parent(&start_node, neighbour, goal);
                open.push(node);
            }

            closed.push(start_node);
        }

        //Examine the nodes
        while let Some(node_current) = open.pop() {
            //If this is the target node return the distance to get there
            if node_current.position == goal {
                //Push all remaining to closed
                for node in open {
                    closed.push(node);
                }

                //Unwind
                let path = unwind(&node_current, &closed);
                return Some(Route {
                    distance: node_current.g,
                    steps: path,
                });
            }

            //Check if node is on closed list and continue if is
            if closed.contains(&node_current) {
                continue;
            }

            //Calculate direction - needs cleaning
            let mut direction_x = node_current.position.0 as i32 - node_current.parent.0 as i32;
            let mut direction_y = node_current.position.1 as i32 - node_current.parent.1 as i32;
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

            if let Some(nodes) = check_jump(&node_current, map, (direction_x, direction_y), goal) {
                for node in nodes {
                    open.push(node);
                }
            }

            //Push current node to closed list
            closed.push(node_current);
        }

        None
    }
}
