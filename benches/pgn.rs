use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rpgn::samples::pgn_samples;
use rpgn::Pgn;

pub fn to_pgn(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_pgn");

    for pgn in pgn_samples().iter().filter_map(|s| s.parsed.as_ref().ok()) {
        let turns = pgn.root_variation.as_ref().unwrap().turns();
        let mut id = String::with_capacity(5 * 2 + 1);

        id.push_str(&turns.first().unwrap().r#move().to_string().replace('-', ""));
        id.push('-');
        id.push_str(&turns.last().unwrap().r#move().to_string().replace('-', ""));

        group.bench_with_input(BenchmarkId::from_parameter(id), &pgn, |b, pgn| {
            b.iter(|| pgn.to_string())
        });
    }

    group.finish();
}

pub fn from_pgn(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_pgn");

    for (pgn_string, pgn) in pgn_samples()
        .iter()
        .filter_map(|s| s.parsed.as_ref().ok().map(|p| (s.string, p)))
    {
        let turns = pgn.root_variation.as_ref().unwrap().turns();
        let mut id = String::with_capacity(5 * 2 + 1);

        id.push_str(&turns.first().unwrap().r#move().to_string().replace('-', ""));
        id.push('-');
        id.push_str(&turns.last().unwrap().r#move().to_string().replace('-', ""));

        group.bench_with_input(
            BenchmarkId::from_parameter(id),
            &pgn_string,
            |b, pgn_string| b.iter(|| Pgn::from_str(pgn_string)),
        );
    }

    group.finish();
}

criterion_group!(benches, to_pgn, from_pgn);
criterion_main!(benches);
