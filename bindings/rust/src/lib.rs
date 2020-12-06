// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

include!("bindings.rs");

pub fn sha256_init(sctx: *mut sha256_state) {
    unsafe {
        (*sctx).state = [SHA256_H0, SHA256_H1, SHA256_H2, SHA256_H3, SHA256_H4, SHA256_H5, SHA256_H6, SHA256_H7];
        (*sctx).count = 0;
    }
}

pub fn sha256_new() -> sha256_state {
    sha256_state {
        state: [SHA256_H0, SHA256_H1, SHA256_H2, SHA256_H3, SHA256_H4, SHA256_H5, SHA256_H6, SHA256_H7],
        count: 0,
        buf: [0; 64],
    }
}
