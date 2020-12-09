// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub use digest::{self, Digest};
use digest::consts::{U32, U64};
use digest::generic_array::GenericArray;
use digest::{BlockInput, FixedOutputDirty, Reset, Update};

include!("bindings.rs");

// type BlockSize = U64;

/// Structure that keeps state of the Sha-256 operation and
/// contains the logic necessary to perform the final calculations.
#[derive(Clone)]
struct Engine256 {
    state: Sha256State,
}

impl Default for Engine256 {
    fn default() -> Self { Engine256 { state: Default::default() } }
}

impl Engine256 {
    fn new(h: &[u32; SHA256_STATE_LEN]) -> Engine256 {
        Engine256 {
            state: Sha256State {
                state: *h,
                count: 0,
                // TODO: Use uninitialize buf
                buf: [0; SHA256_BLOCK_SIZE],
            },
        }
    }

    fn update(&mut self, input: &[u8]) {
        // Assumes that input.len() can be converted to u64 without overflow
        // self.len += (input.len() as u64) << 3;
        // let s = &mut self.state;
        // self.buffer.input_blocks(input, |b| compress256(s, b));
        unsafe {
            sha256_update(&mut self.state, input.as_ptr(), input.len() as u32);
        }
    }

    fn finish(&mut self) {
        // let s = &mut self.state;
        // let l = self.len;
        // self.buffer
        //     .len64_padding_be(l, |b| compress256(s, from_ref(b)));

        let mut _out: [u8; 32] = [0; 32];
        unsafe {
            sha256_final(&mut self.state, _out.as_mut_ptr());
        }
    }

    // fn reset(&mut self, h: &[u32; SHA256_STATE_LEN]) {
    //     self.state = Sha256State {
    //         state: *h,
    //         count: 0,
    //         // TODO: Use uninitialize buf
    //         buf: [0; SHA256_BLOCK_SIZE],
    //     }
    // }
}

#[derive(Clone)]
pub struct Sha256 {
    engine: Engine256,
}

impl Sha256 {
    pub fn input(&mut self, blocks: &[&[u8]]) {
        debug_assert_eq!(blocks.len() % 2, 0, "invalid block length");

        for block in blocks.iter() {
            self.engine.update(block);
        }
    }

    pub fn finish(&mut self) -> [u8; 32] {
        let mut out = [0u8; 32];
        unsafe {
            sha256_final(&mut self.engine.state, out.as_mut_ptr());
        }
        out
    }

    pub fn finish_with(mut self, block0: &[u8]) -> [u8; 32] {
        debug_assert_eq!(block0.len(), 32);

        self.engine.update(block0);

        let mut out = [0u8; 32];
        unsafe {
            sha256_final(&mut self.engine.state, out.as_mut_ptr());
        }
        out
    }
}

impl Default for Sha256 {
    fn default() -> Self { Sha256 { engine: Default::default() } }
}

impl BlockInput for Sha256 {
    type BlockSize = U64;
}

impl Update for Sha256 {
    fn update(&mut self, input: impl AsRef<[u8]>) { self.engine.update(input.as_ref()); }
}

impl FixedOutputDirty for Sha256 {
    type OutputSize = U32;

    fn finalize_into_dirty(&mut self, out: &mut digest::Output<Self>) {
        // unsafe {
        //     sha256_final(&mut self, out.as_mut_ptr());
        // }
        self.engine.finish();
        let s = &self.engine.state.state[..8];
        for (chunk, v) in out.chunks_exact_mut(4).zip(s.iter()) {
            chunk.copy_from_slice(&v.to_be_bytes());
        }
    }
}

impl Reset for Sha256 {
    fn reset(&mut self) { self.engine = Default::default(); }
}

opaque_debug::implement!(Sha256);

digest::impl_write!(Sha256);

// #[target_feature(enable = "sha,sse2,ssse3,sse4.1")]
//     if cpuid_bool::cpuid_bool!("sha", "sse2", "ssse3", "sse4.1") {

pub fn compress256(state: &mut [u32; SHA256_STATE_LEN],
                   blocks: &[GenericArray<u8, U64>]) {
    // SAFETY: GenericArray<u8, U64> and [u8; 64] have
    // exactly the same memory layout
    #[allow(unsafe_code)]
    let mut engine = Engine256::new(state);
    let blocks: &[[u8; 64]] = unsafe { &*(blocks as *const _ as *const [[u8; 64]]) };
    (*blocks).iter().for_each(|block| engine.update(block));
    *state = engine.state.state;
}

// #[cfg(feature = "compress")]
// pub use sha256fast::compress256;
