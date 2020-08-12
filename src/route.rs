use std::convert::From;

use movingai::Coords2D;

///Describes a route between to points.
///Giving the total distance needed to travel and a vector of each step needed.
pub struct Route {
    distance: f64,
    steps: Vec<Coords2D>,
}

impl From<(f64, Vec<Coords2D>)> for Route {
    fn from(item: (f64, Vec<Coords2D>)) -> Self {
        Route {
            distance: item.0,
            steps: item.1,
        }
    }
}

impl Route {
    ///Returns a vector of Coords2D, each representing a step in the path.
    ///Organised in reverse order (destination is at [0]) to allow calling .pop() to get each step.
    pub fn steps(&self) -> Vec<Coords2D> {
        self.steps.clone()
    }

    ///Returns the total distance needed to travel as an f64.
    pub fn distance(&self) -> f64 {
        self.distance
    }
}
