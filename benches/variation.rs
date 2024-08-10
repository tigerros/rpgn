use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use rpgn::samples::*;
use rpgn::{Pgn, Turn};

pub fn construct(c: &mut Criterion) {
    let mut group = c.benchmark_group("construct");
    
    for var in variation_sample_fns() {
        let turns = var().turns();
        let mut id = String::with_capacity(4 * 2 + 1);

        id.push_str(&turns.first().unwrap().r#move().to_string().replace('-', ""));
        id.push('-');
        id.push_str(&turns.last().unwrap().r#move().to_string().replace('-', ""));

        group.bench_with_input(BenchmarkId::from_parameter(id), &(), |b, ()| {
            b.iter(|| black_box(var()))
        });
    }
    
    group.finish()
}

pub fn read_positions(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_positions");

    for var in variation_sample_fns() {
        let var = var();
        let turns = var.turns();
        let mut id = String::with_capacity(4 * 2 + 1);

        id.push_str(&turns.first().unwrap().r#move().to_string().replace('-', ""));
        id.push('-');
        id.push_str(&turns.last().unwrap().r#move().to_string().replace('-', ""));

        group.bench_with_input(BenchmarkId::from_parameter(id), &(turns, var), |b, (turns, var)| {
            b.iter(|| {
                for i in 0..turns.len() {
                    let _ = var.get_position(i);
                }
            })
        });
    }

    group.finish();
}

criterion_group!(benches, read_positions);
criterion_main!(benches);
