#[cfg(test)]
mod tests {
    use std::path::Path;

    use movingai::parser::parse_map_file;
    use movingai::parser::parse_scen_file;

    use blitz_path::{a_star_path, jps_path};

    const MAP: &str = "./tests/map/maze512-32-9.map";
    const SCEN: &str = "./tests/map/maze512-32-9.map.scen";

    enum Algorithm {
        AStar,
        JPS,
    }

    fn test_scen(algorithm: Algorithm, map: &str, scen: &str) -> Vec<String> {
        let mut errors = Vec::new();
        let map = parse_map_file(Path::new(map)).unwrap();
        let scenes = parse_scen_file(Path::new(scen)).unwrap();

        for (i, scene) in scenes.iter().enumerate() {
            let path = match algorithm {
                Algorithm::AStar => a_star_path(&map, scene.start_pos, scene.goal_pos),
                Algorithm::JPS => jps_path(&map, scene.start_pos, scene.goal_pos),
            };

            match path {
                None => {
                    //panic!("Test failed at scen #{}", i);
                    errors.push(format!("Test #{} found no path", i));
                }
                Some(path) => {
                    for num in 0..path.steps().len() - 1 {
                        //Test wheter the x or y diustance between any adjacent steps is more than 1
                        let direction_x =
                            path.steps()[num + 1].0 as i32 - path.steps()[num].0 as i32;
                        let direction_y =
                            path.steps()[num + 1].1 as i32 - path.steps()[num].1 as i32;
                        if direction_x < -1
                            || direction_x > 1
                            || direction_y < -1
                            || direction_y > 1
                        {
                            errors.push(format!("Test #{} did not unwind correctly", i))
                        }
                    }
                }
            }
        }

        errors
    }

    #[test]
    fn jps() {
        let errors = test_scen(Algorithm::JPS, MAP, SCEN);

        assert!(
            errors.len() < 1,
            "The following tests failed:\n{:?}",
            errors
        );
    }

    #[test]
    fn a_star() {
        let errors = test_scen(Algorithm::AStar, MAP, SCEN);

        assert!(
            errors.len() < 1,
            "The following tests failed:\n{:?}",
            errors
        );
    }
}
