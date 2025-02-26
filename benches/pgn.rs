use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rpgn::samples::{sans0, sans1, variation0, variation1, variation2, PgnSample};
use rpgn::Pgn;
use rpgn::{Sans, Variation};
use shakmaty::san::SanPlus;

fn sans_samples() -> [PgnSample<Sans<SanPlus>>; 2] {
    [sans0(), sans1()]
}

fn variation_samples() -> [PgnSample<Variation<SanPlus>>; 3] {
    [variation0(), variation1(), variation2()]
}

pub fn to_pgn(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_pgn");

    for pgn in sans_samples().iter().filter_map(|s| s.parsed.as_ref().ok()) {
        let id = format!(
            "Simple {}-{}",
            pgn.movetext.0.first().unwrap().to_string().replace('-', ""),
            pgn.movetext.0.last().unwrap().to_string().replace('-', "")
        );

        group.bench_with_input(BenchmarkId::from_parameter(id), &pgn, |b, pgn| {
            b.iter(|| pgn.to_string())
        });
    }

    for pgn in variation_samples()
        .iter()
        .filter_map(|s| s.parsed.as_ref().ok())
    {
        let id = format!(
            "Variation {}-{}",
            pgn.movetext
                .0
                .first()
                .unwrap()
                .san
                .to_string()
                .replace('-', ""),
            pgn.movetext
                .0
                .last()
                .unwrap()
                .san
                .to_string()
                .replace('-', "")
        );

        group.bench_with_input(BenchmarkId::from_parameter(id), &pgn, |b, pgn| {
            b.iter(|| pgn.to_string())
        });
    }

    group.finish();
}

pub fn from_pgn(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_pgn");

    for (pgn_string, pgn) in sans_samples()
        .iter()
        .filter_map(|s| s.parsed.as_ref().ok().map(|p| (s.string, p)))
    {
        let id = format!(
            "Simple {}-{}",
            pgn.movetext.0.first().unwrap().to_string().replace('-', ""),
            pgn.movetext.0.last().unwrap().to_string().replace('-', "")
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(id),
            &pgn_string,
            |b, pgn_string| b.iter(|| Pgn::<Sans<SanPlus>>::from_str(pgn_string)),
        );
    }

    for (pgn_string, pgn) in variation_samples()
        .iter()
        .filter_map(|s| s.parsed.as_ref().ok().map(|p| (s.string, p)))
    {
        let id = format!(
            "Variation {}-{}",
            pgn.movetext
                .0
                .first()
                .unwrap()
                .san
                .to_string()
                .replace('-', ""),
            pgn.movetext
                .0
                .last()
                .unwrap()
                .san
                .to_string()
                .replace('-', "")
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(id),
            &pgn_string,
            |b, pgn_string| b.iter(|| Pgn::<Variation<SanPlus>>::from_str(pgn_string)),
        );
    }

    group.finish();
}

criterion_group!(benches, to_pgn, from_pgn);
criterion_main!(benches);
