// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use sha256fast::*;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_abc(c: &mut Criterion) {
    let data = "abc";
    let mut out: [u8; 32] = [0; 32];

    c.bench_function("fib 20", |b| {
        b.iter(|| unsafe {
                   let mut sctx = sha256_new();
                   sha256_update(&mut sctx, data.as_ptr(), 3);
                   sha256_final(&mut sctx, out.as_mut_ptr());
               })
    });

}

criterion_group!(benches, bench_abc);

criterion_main!(benches);
