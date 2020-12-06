// TODO: Use rust-bindgen

const SHA256_DIGEST_SIZE: usize = 32;
const SHA256_BLOCK_SIZE: usize = 64;

const SHA256_H0: u32 = 0x6a09e667;
const SHA256_H1: u32 = 0xbb67ae85;
const SHA256_H2: u32 = 0x3c6ef372;
const SHA256_H3: u32 = 0xa54ff53a;
const SHA256_H4: u32 = 0x510e527f;
const SHA256_H5: u32 = 0x9b05688c;
const SHA256_H6: u32 = 0x1f83d9ab;
const SHA256_H7: u32 = 0x5be0cd19;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct sha256_state {
    pub state: [u32; SHA256_DIGEST_SIZE / 4],
    pub count: u64,
    pub buf: [u8; SHA256_BLOCK_SIZE],
}

extern "C" {
    pub fn sha256_update(sctx: *mut sha256_state, data: *const u8, len: u32);
    pub fn sha256_final(sctx: *mut sha256_state, out: *mut u8);
}
