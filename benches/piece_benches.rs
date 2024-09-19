#![allow(unused_imports)]
#![allow(dead_code)]

use criterion::{
    BenchmarkId,
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use aster::types::{
    bitboard::Bitboard,
    square::Square,
    piece::Piece
};
use aster::legal_moves::{
    n_move_gen,
    k_move_gen,
    b_move_gen,
    r_move_gen,
    q_move_gen
};

const LOOKUP_VALUES: [(Square, &str); 3] = [
    (Square::A1, "corner"),
    (Square::A4, "edge"),
    (Square::E4, "center"),
];

const SLIDER_VALUES: [(Square, Bitboard, &str); 4] = [
    (Square::A1, Bitboard(0x0000000000000000), "corner_empty"),
    (Square::A1, Bitboard(0x3104a4008820e094), "corner_pop"),
    (Square::E4, Bitboard(0x0000000000000000), "center_empty"),
    (Square::E4, Bitboard(0x2c02740609802491), "center_pop")
];

pub fn n_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("knight_moves");
    let mut bench_square = |sq: Square, name: &'static str| {
        group.bench_function(name, |b| b.iter(|| n_move_gen(black_box(sq))));
    };

    for metric in LOOKUP_VALUES {
        bench_square(metric.0, metric.1);
    }
}

pub fn k_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("king_moves");
    let mut bench_square = |sq: Square, name: &'static str| {
        group.bench_function(name, |b| b.iter(|| k_move_gen(black_box(sq))));
    };

    for metric in LOOKUP_VALUES {
        bench_square(metric.0, metric.1);
    }
}

pub fn b_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("bishop_moves");
    let mut bench_square = |sq: Square, blockers: Bitboard, name: &'static str| {
        group.bench_function(name, |b| b.iter(|| b_move_gen(black_box(sq), black_box(blockers))));
    };

    for metric in SLIDER_VALUES {
        bench_square(metric.0, metric.1, metric.2);
    }
}


pub fn r_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("rook_moves");
    let mut bench_square = |sq: Square, blockers: Bitboard, name: &'static str| {
        group.bench_function(name, |b| b.iter(|| r_move_gen(black_box(sq), black_box(blockers))));
    };

    for metric in SLIDER_VALUES {
        bench_square(metric.0, metric.1, metric.2);
    }
}

pub fn q_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("queen_moves");
    let mut bench_square = |sq: Square, blockers: Bitboard, name: &'static str| {
        group.bench_function(name, |b| b.iter(|| q_move_gen(black_box(sq), black_box(blockers))));
    };

    for metric in SLIDER_VALUES {
        bench_square(metric.0, metric.1, metric.2);
    }
}

criterion_group!(benches,
    n_benches,
    k_benches,
    b_benches,
    r_benches,
    q_benches
);
criterion_main!(benches);
