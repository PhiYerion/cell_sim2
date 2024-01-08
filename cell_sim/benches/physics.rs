use cell_sim::cell::Cell;
use cell_sim::physics::World;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nalgebra::vector;

pub fn criterion_benchmark(c: &mut Criterion) {
    test_physics(c, 256, 64);
    test_cells(c, 256, 64);

    test_physics(c, 64, 256);
    test_cells(c, 64, 256);

    test_all(c, 1024, 64);
}

fn test_cells(c: &mut Criterion, rounds: usize, cells: usize) {
    let string = format!("cells x{} rounds x{}", cells, rounds);
    c.bench_function(string.as_str(), |b| {
        b.iter(|| {
            let mut world = World::default();
            (0..black_box(cells)).for_each(|_| {
                world.add_cell(Cell::new_random(), vector![rand::random(), rand::random()]);
            });

            (0..black_box(rounds)).for_each(|_| {
                world.update_cells();
            })
        })
    });
}

fn test_physics(c: &mut Criterion, rounds: usize, cells: usize) {
    let string = format!("physics x{} cells x{} rounds", cells, rounds);
    c.bench_function(string.as_str(), |b| {
        b.iter(|| {
            let mut world = World::default();
            (0..black_box(cells)).for_each(|_| {
                world.add_cell(Cell::new_random(), vector![rand::random(), rand::random()]);
            });

            (0..black_box(rounds)).for_each(|_| {
                world.update_physics();
            })
        })
    });
}

fn test_all(c: &mut Criterion, rounds: usize, cells: usize) {
    let string = format!("all x{} cells x{} rounds", cells, rounds);
    c.bench_function(string.as_str(), |b| {
        b.iter(|| {
            let mut world = World::default();
            (0..black_box(cells)).for_each(|_| {
                world.add_cell(Cell::new_random(), vector![rand::random(), rand::random()]);
            });

            (0..black_box(rounds)).for_each(|_| {
                world.update();
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
