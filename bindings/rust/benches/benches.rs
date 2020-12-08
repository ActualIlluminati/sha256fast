// Copyright Deni Sukhonina
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use sha256fast::{Digest, Sha256};

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_abc(c: &mut Criterion) {
    let data = b"abc";

    c.bench_function("sha256: abc", |b| b.iter(|| { let _ = Sha256::digest(data); }));
}

criterion_group!(benches, bench_abc);

criterion_main!(benches);
