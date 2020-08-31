use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use movingai::parser::parse_map_file;
use movingai::parser::parse_scen_file;
use movingai::{MovingAiMap, SceneRecord};
use std::path::Path;

use blitz_path::{a_star_path, jps_path};

const MAP: &str = "./tests/map/maze512-32-9.map";
const SCEN: &str = "./tests/map/maze512-32-9.map.scen";

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("maze512-32-9");
    let (map, scenes) = load_files();
    for i in [0, 34].iter() {
        let scene = scenes[*i as usize].clone();
        group.bench_with_input(BenchmarkId::new("A*", i), &scene, |b, scene| {
            b.iter(|| {
                a_star_path(
                    black_box(&map),
                    black_box(scene.start_pos),
                    black_box(scene.goal_pos),
                )
            })
        });

        group.bench_with_input(BenchmarkId::new("JPS", i), &scene, |b, scene| {
            b.iter(|| {
                jps_path(
                    black_box(&map),
                    black_box(scene.start_pos),
                    black_box(scene.goal_pos),
                )
            })
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn load_files() -> (MovingAiMap, Vec<SceneRecord>) {
    let map = parse_map_file(Path::new(MAP)).expect("Could not load map");
    let scenes = parse_scen_file(Path::new(SCEN)).expect("Could not load scenes");

    (map, scenes)
}
