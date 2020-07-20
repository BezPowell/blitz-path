use std::convert::From;

use movingai::Coords2D;

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
    pub fn steps(&self) -> Vec<Coords2D> {
        self.steps.clone()
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}
