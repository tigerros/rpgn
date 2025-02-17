use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rpgn::samples::{san_vec0, san_vec1, variation0, variation1, variation2, PgnSample};
use rpgn::Pgn;
use rpgn::{SanVec, Variation, VariationMovetext};

fn san_vec_samples() -> [PgnSample<SanVec>; 2] {
    [san_vec0(), san_vec1()]
}

fn variation_samples() -> [PgnSample<Variation>; 3] {
    [variation0(), variation1(), variation2()]
}

pub fn to_pgn(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_pgn");

    for pgn in san_vec_samples()
        .iter()
        .filter_map(|s| s.parsed.as_ref().ok())
    {
        let Some(movetext) = &pgn.movetext else {
            continue;
        };
        let id = format!(
            "Simple {}-{}",
            movetext.0.first().unwrap().to_string().replace('-', ""),
            movetext.0.last().unwrap().to_string().replace('-', "")
        );

        group.bench_with_input(BenchmarkId::from_parameter(id), &pgn, |b, pgn| {
            b.iter(|| pgn.to_string())
        });
    }

    for pgn in variation_samples()
        .iter()
        .filter_map(|s| s.parsed.as_ref().ok())
    {
        let Some(movetext) = &pgn.movetext else {
            continue;
        };
        let id = format!(
            "Variation {}-{}",
            movetext.0.first().unwrap().san.to_string().replace('-', ""),
            movetext.0.last().unwrap().san.to_string().replace('-', "")
        );

        group.bench_with_input(BenchmarkId::from_parameter(id), &pgn, |b, pgn| {
            b.iter(|| pgn.to_string())
        });
    }

    group.finish();
}

pub fn from_pgn(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_pgn");

    for (pgn_string, pgn) in san_vec_samples()
        .iter()
        .filter_map(|s| s.parsed.as_ref().ok().map(|p| (s.string, p)))
    {
        let Some(movetext) = &pgn.movetext else {
            continue;
        };
        let id = format!(
            "Simple {}-{}",
            movetext.0.first().unwrap().to_string().replace('-', ""),
            movetext.0.last().unwrap().to_string().replace('-', "")
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(id),
            &pgn_string,
            |b, pgn_string| b.iter(|| Pgn::from_str::<SanVec>(pgn_string)),
        );
    }

    for (pgn_string, pgn) in variation_samples()
        .iter()
        .filter_map(|s| s.parsed.as_ref().ok().map(|p| (s.string, p)))
    {
        let Some(movetext) = &pgn.movetext else {
            continue;
        };
        let id = format!(
            "Variation {}-{}",
            movetext.0.first().unwrap().san.to_string().replace('-', ""),
            movetext.0.last().unwrap().san.to_string().replace('-', "")
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(id),
            &pgn_string,
            |b, pgn_string| b.iter(|| Pgn::from_str::<VariationMovetext>(pgn_string)),
        );
    }

    group.finish();
}

criterion_group!(benches, to_pgn, from_pgn);
criterion_main!(benches);
