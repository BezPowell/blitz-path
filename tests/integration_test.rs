#[cfg(test)]
mod tests {
    use std::path::Path;

    use movingai::parser::parse_map_file;
    use movingai::parser::parse_scen_file;
    use movingai::{MovingAiMap, SceneRecord};

    use blitz_path::{a_star_path, jps_path};

    const MAP: &str = "./tests/map/maze512-32-9.map";
    const SCEN: &str = "./tests/map/maze512-32-9.map.scen";
    const TEST_NUMS_JPS: [usize; 3] = [0, 34, 1740];
    const TEST_NUMS_A_STAR: [usize; 2] = [0, 34];

    enum Algorithm {
        AStar,
        JPS,
    }

    fn test_scen(
        algorithm: Algorithm,
        map: &MovingAiMap,
        scen: &Vec<SceneRecord>,
        tests: Vec<usize>,
    ) -> Vec<String> {
        let mut errors = Vec::new();

        for index in tests {
            let scene = &scen[index];
            let path = match algorithm {
                Algorithm::AStar => a_star_path(map, scene.start_pos, scene.goal_pos),
                Algorithm::JPS => jps_path(map, scene.start_pos, scene.goal_pos),
            };

            match path {
                None => {
                    errors.push(format!("Test #{} found no path", index));
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
                            errors.push(format!("Test #{} did not unwind correctly", index))
                        }
                    }
                }
            }
        }

        errors
    }

    #[test]
    fn jps() {
        let map = parse_map_file(Path::new(MAP)).unwrap();
        let scenes = parse_scen_file(Path::new(SCEN)).unwrap();
        let tests = TEST_NUMS_JPS.to_vec();

        let errors = test_scen(Algorithm::JPS, &map, &scenes, tests);

        assert!(
            errors.len() < 1,
            "The following tests failed:\n{:?}",
            errors
        );
    }

    #[test]
    #[ignore = "very slow"]
    fn jps_full() {
        let map = parse_map_file(Path::new(MAP)).unwrap();
        let scenes = parse_scen_file(Path::new(SCEN)).unwrap();
        let all_tests = (0..scenes.len()).collect();

        let errors = test_scen(Algorithm::JPS, &map, &scenes, all_tests);

        assert!(
            errors.len() < 1,
            "The following tests failed:\n{:?}",
            errors
        );
    }

    #[test]
    fn a_star() {
        let map = parse_map_file(Path::new(MAP)).unwrap();
        let scenes = parse_scen_file(Path::new(SCEN)).unwrap();
        let tests = TEST_NUMS_A_STAR.to_vec();

        let errors = test_scen(Algorithm::AStar, &map, &scenes, tests);

        assert!(
            errors.len() < 1,
            "The following tests failed:\n{:?}",
            errors
        );
    }

    #[test]
    #[ignore = "extremely slow"]
    fn a_star_full() {
        let map = parse_map_file(Path::new(MAP)).unwrap();
        let scenes = parse_scen_file(Path::new(SCEN)).unwrap();
        let all_tests = (0..scenes.len()).collect();

        let errors = test_scen(Algorithm::AStar, &map, &scenes, all_tests);

        assert!(
            errors.len() < 1,
            "The following tests failed:\n{:?}",
            errors
        );
    }
}
