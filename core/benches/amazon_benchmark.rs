use amazons_core::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn minimax_benchmark(c: &mut Criterion) {
    let board = Board::default();
    c.bench_function("minimax depth 2", |b| {
        b.iter(|| {
            _minimax(
                black_box(&board),
                black_box(2),
                black_box(true),
                black_box(MMT::MIN),
                black_box(MMT::MAX),
                black_box(&mut 0),
                black_box(None),
            )
        })
    });
}

fn reachable_benchmark(c: &mut Criterion) {
    let board = Board::default();
    c.bench_function("reachable from one starting piece", |b| {
        b.iter(|| {
            for c in board.reachable_squares(black_box(&board.pieces[0])) {
                black_box(c);
            }
        })
    });
    c.bench_function("reachable from all starting pieces", |b| {
        b.iter(|| {
            for coord in board.pieces {
                for c in board.reachable_squares(black_box(&coord)) {
                    black_box(c);
                }
            }
        })
    });
    c.bench_function("reachable from all squares", |b| {
        b.iter(|| {
            for coord in 0..100 {
                for c in board.reachable_squares(&Coord::from(black_box(coord))) {
                    black_box(c);
                }
            }
        })
    });
}

fn heuristic_benchmark(c: &mut Criterion) {
    let board = Board::default();
    c.bench_function("heuristic empty board", |b| {
        b.iter(|| black_box(better_reachable_heuristic(black_box(&board))))
    });
}

criterion_group!(
    benches,
    minimax_benchmark,
    reachable_benchmark,
    heuristic_benchmark
);
criterion_main!(benches);
