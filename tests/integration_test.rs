#[cfg(test)]
mod tests {
    use std::path::Path;

    use movingai::parser::parse_map_file;
    use movingai::parser::parse_scen_file;

    use blitz_path::{a_star_path, jps_path};

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
                _ => {}
            }
        }

        errors
    }

    #[test]
    fn jps() {
        let errors = test_scen(
            Algorithm::JPS,
            "./tests/map/maze512-32-9.map",
            "./tests/map/maze512-32-9.map.scen",
        );

        assert!(
            errors.len() < 1,
            "The following tests failed:\n{:?}",
            errors
        );
    }

    #[test]
    fn a_star() {
        let errors = test_scen(
            Algorithm::AStar,
            "./tests/map/maze512-32-9.map",
            "./tests/map/maze512-32-9.map.scen",
        );

        assert!(
            errors.len() < 1,
            "The following tests failed:\n{:?}",
            errors
        );
    }
}
