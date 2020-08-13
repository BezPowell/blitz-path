use criterion::{black_box, criterion_group, criterion_main, Criterion};
use movingai::parser::parse_map_file;
use movingai::parser::parse_scen_file;
use movingai::{MovingAiMap, SceneRecord};
use std::path::Path;

use blitz_path::a_star_path;

const MAP: &str = "./tests/map/maze512-32-9.map";
const SCEN: &str = "./tests/map/maze512-32-9.map.scen";

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("a* 0", |b| {
        let (map, scenes) = load_files();
        let scenes = vec![scenes[0].clone()];

        b.iter(|| bench_scenes(black_box(&map), black_box(&scenes)))
    });

    c.bench_function("a* 34", |b| {
        let (map, scenes) = load_files();
        let scenes = vec![scenes[34].clone()];

        b.iter(|| bench_scenes(black_box(&map), black_box(&scenes)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn bench_scenes(map: &MovingAiMap, scenes: &Vec<SceneRecord>) {
    for scene in scenes {
        a_star_path(&map, scene.start_pos, scene.goal_pos);
    }
}

fn load_files() -> (MovingAiMap, Vec<SceneRecord>) {
    let map = parse_map_file(Path::new(MAP)).expect("Could not load map");
    let scenes = parse_scen_file(Path::new(SCEN)).expect("Could not load scenes");

    (map, scenes)
}
