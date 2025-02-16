use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rpgn::samples::{simple0, simple1, variation0, variation1, variation2, PgnSample};
use rpgn::Pgn;
use rpgn::{SimpleMovetext, VariationMovetext, VariationMovetextImpl};

fn simple_samples() -> [PgnSample<SimpleMovetext>; 2] {
    [simple0(), simple1()]
}

fn variation_samples() -> [PgnSample<VariationMovetext>; 3] {
    [variation0(), variation1(), variation2()]
}

pub fn to_pgn(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_pgn");

    for pgn in simple_samples()
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
            movetext.0.first().unwrap().0.to_string().replace('-', ""),
            movetext.0.last().unwrap().0.to_string().replace('-', "")
        );

        group.bench_with_input(BenchmarkId::from_parameter(id), &pgn, |b, pgn| {
            b.iter(|| pgn.to_string())
        });
    }

    group.finish();
}

pub fn from_pgn(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_pgn");

    for (pgn_string, pgn) in simple_samples()
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
            |b, pgn_string| b.iter(|| Pgn::from_str::<SimpleMovetext>(pgn_string)),
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
            movetext.0.first().unwrap().0.to_string().replace('-', ""),
            movetext.0.last().unwrap().0.to_string().replace('-', "")
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(id),
            &pgn_string,
            |b, pgn_string| b.iter(|| Pgn::from_str::<VariationMovetextImpl>(pgn_string)),
        );
    }

    group.finish();
}

criterion_group!(benches, to_pgn, from_pgn);
criterion_main!(benches);
