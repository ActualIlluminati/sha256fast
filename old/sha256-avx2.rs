#![no_main]

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

const STATE_LEN: usize = 8;

const H256: [u32; STATE_LEN] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                                0x5be0cd19];
const K32: [u32; 64] = [0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
                        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
                        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
                        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
                        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
                        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
                        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
                        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
                        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2];

#[inline(always)]
pub unsafe fn k(x: u32) -> __m256i { _mm256_set1_epi32(x as i32) }

#[inline(always)]
pub unsafe fn add(x: __m256i, y: __m256i) -> __m256i { _mm256_add_epi32(x, y) }
#[inline(always)]
pub unsafe fn add3(x: __m256i, y: __m256i, z: __m256i) -> __m256i { add(add(x, y), z) }
#[inline(always)]
pub unsafe fn add4(x: __m256i, y: __m256i, z: __m256i, w: __m256i) -> __m256i { add(add(x, y), add(z, w)) }
#[inline(always)]
pub unsafe fn add5(x: __m256i, y: __m256i, z: __m256i, w: __m256i, v: __m256i) -> __m256i {
    add(add3(x, y, z), add(w, v))
}

#[inline(always)]
pub unsafe fn inc(x: &mut __m256i, y: __m256i) -> __m256i {
    *x = add(*x, y);
    *x
}
#[inline(always)]
pub unsafe fn inc3(x: &mut __m256i, y: __m256i, z: __m256i) -> __m256i {
    *x = add3(*x, y, z);
    *x
}
#[inline(always)]
pub unsafe fn inc4(x: &mut __m256i, y: __m256i, z: __m256i, w: __m256i) -> __m256i {
    *x = add4(*x, y, z, w);
    *x
}

#[inline(always)]
pub unsafe fn xor(x: __m256i, y: __m256i) -> __m256i { _mm256_xor_si256(x, y) }
#[inline(always)]
pub unsafe fn xor3(x: __m256i, y: __m256i, z: __m256i) -> __m256i { xor(xor(x, y), z) }
#[inline(always)]
pub unsafe fn or(x: __m256i, y: __m256i) -> __m256i { _mm256_or_si256(x, y) }
#[inline(always)]
pub unsafe fn and(x: __m256i, y: __m256i) -> __m256i { _mm256_and_si256(x, y) }

#[inline(always)]
pub unsafe fn shr(x: __m256i, n: i32) -> __m256i { _mm256_srli_epi32(x, n) }
#[inline(always)]
pub unsafe fn shl(x: __m256i, n: i32) -> __m256i { _mm256_slli_epi32(x, n) }

#[inline(always)]
pub unsafe fn ch(x: __m256i, y: __m256i, z: __m256i) -> __m256i { xor(z, and(x, xor(y, z))) }
#[inline(always)]
pub unsafe fn maj(x: __m256i, y: __m256i, z: __m256i) -> __m256i { or(and(x, y), and(z, or(x, y))) }
#[inline(always)]
pub unsafe fn big_sigma0(x: __m256i) -> __m256i {
    xor3(or(shr(x, 2), shl(x, 30)), or(shr(x, 13), shl(x, 19)), or(shr(x, 22), shl(x, 10)))
}
#[inline(always)]
pub unsafe fn big_sigma1(x: __m256i) -> __m256i {
    xor3(or(shr(x, 6), shl(x, 26)), or(shr(x, 11), shl(x, 21)), or(shr(x, 25), shl(x, 7)))
}
#[inline(always)]
pub unsafe fn sigma0(x: __m256i) -> __m256i { xor3(or(shr(x, 7), shl(x, 25)), or(shr(x, 18), shl(x, 14)), shr(x, 3)) }
#[inline(always)]
pub unsafe fn sigma1(x: __m256i) -> __m256i { xor3(or(shr(x, 17), shl(x, 15)), or(shr(x, 19), shl(x, 13)), shr(x, 10)) }

#[inline(always)]
pub unsafe fn round(a: __m256i,
                    b: __m256i,
                    c: __m256i,
                    d: &mut __m256i,
                    e: __m256i,
                    f: __m256i,
                    g: __m256i,
                    h: &mut __m256i,
                    k: __m256i) {
    let t1: __m256i = add4(*h, big_sigma1(e), ch(e, f, g), k);
    let t2: __m256i = add(big_sigma0(a), maj(a, b, c));
    *d = add(*d, t1);
    *h = add(t1, t2);
}

#[inline(always)]
pub unsafe fn read_le32(ptr: *const u8) -> u32 { u32::from_le_bytes(*(ptr as *const [u8; 4])) }

#[inline(always)]
pub unsafe fn read_be32(ptr: *const u8) -> u32 { u32::from_be_bytes(*(ptr as *const [u8; 4])) }

#[inline(always)]
pub unsafe fn write_le32(ptr: *mut u8, x: u32) { *(ptr as *mut [u8; 4]) = x.to_le_bytes(); }

#[inline(always)]
pub unsafe fn write_be32(ptr: *mut u8, x: u32) { *(ptr as *mut [u8; 4]) = x.to_be_bytes(); }

#[inline(always)]
pub unsafe fn write_be64(ptr: *mut u8, x: u64) { *(ptr as *mut [u8; 8]) = x.to_be_bytes(); }

#[no_mangle]
#[inline(always)]
pub unsafe fn read8(chunk: &[u8]) -> __m256i {
    let mask = _mm256_set_epi32(0x0C0D0E0Fu32 as i32,
                                0x08090A0Bu32 as i32,
                                0x04050607u32 as i32,
                                0x00010203u32 as i32,
                                0x0C0D0E0Fu32 as i32,
                                0x08090A0Bu32 as i32,
                                0x04050607u32 as i32,
                                0x00010203u32 as i32);
    let ret: __m256i = _mm256_set_epi32(read_le32(chunk.as_ptr().add(0)) as i32,
                                        read_le32(chunk.as_ptr().add(64)) as i32,
                                        read_le32(chunk.as_ptr().add(128)) as i32,
                                        read_le32(chunk.as_ptr().add(192)) as i32,
                                        read_le32(chunk.as_ptr().add(256)) as i32,
                                        read_le32(chunk.as_ptr().add(320)) as i32,
                                        read_le32(chunk.as_ptr().add(384)) as i32,
                                        read_le32(chunk.as_ptr().add(448)) as i32);
    _mm256_shuffle_epi8(ret, mask)
}

#[no_mangle]
#[inline(always)]
pub unsafe fn write8(out: *mut u8, mut v: __m256i) {
    let mask = _mm256_set_epi32(0x0C0D0E0Fu32 as i32,
                                0x08090A0Bu32 as i32,
                                0x04050607u32 as i32,
                                0x00010203u32 as i32,
                                0x0C0D0E0Fu32 as i32,
                                0x08090A0Bu32 as i32,
                                0x04050607u32 as i32,
                                0x00010203u32 as i32);
    v = _mm256_shuffle_epi8(v, mask);
    write_le32(out.add(0), _mm256_extract_epi32(v, 7) as u32);
    write_le32(out.add(32), _mm256_extract_epi32(v, 6) as u32);
    write_le32(out.add(64), _mm256_extract_epi32(v, 5) as u32);
    write_le32(out.add(96), _mm256_extract_epi32(v, 4) as u32);
    write_le32(out.add(128), _mm256_extract_epi32(v, 3) as u32);
    write_le32(out.add(160), _mm256_extract_epi32(v, 2) as u32);
    write_le32(out.add(192), _mm256_extract_epi32(v, 1) as u32);
    write_le32(out.add(224), _mm256_extract_epi32(v, 0) as u32);
}

pub unsafe fn digest_init() -> [__m256i; STATE_LEN] {
    [k(H256[0]), k(H256[1]), k(H256[2]), k(H256[3]), k(H256[4]), k(H256[5]), k(H256[6]), k(H256[7])]
}

pub unsafe fn digest_finish(out: &mut [u8; 32], digest: [__m256i; STATE_LEN]) {
    for i in 0..digest.len() {
        write8(out.as_mut_ptr().add(0), a);
        write8(out.as_mut_ptr().add(4), b);
        write8(out.as_mut_ptr().add(8), c);
        write8(out.as_mut_ptr().add(12), d);
        write8(out.as_mut_ptr().add(16), e);
        write8(out.as_mut_ptr().add(20), f);
        write8(out.as_mut_ptr().add(24), g);
        write8(out.as_mut_ptr().add(28), h);
    }
}

#[inline(always)]
pub unsafe fn transform_8way(digest: &mut [__m256i; STATE_LEN], input: *const u8 /*&[u8; 64*8]*/) {
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *digest;

    let mut w0: __m256i;
    let mut w1: __m256i;
    let mut w2: __m256i;
    let mut w3: __m256i;
    let mut w4: __m256i;
    let mut w5: __m256i;
    let mut w6: __m256i;
    let mut w7: __m256i;
    let mut w8: __m256i;
    let mut w9: __m256i;
    let mut w10: __m256i;
    let mut w11: __m256i;
    let mut w12: __m256i;
    let mut w13: __m256i;
    let mut w14: __m256i;
    let mut w15: __m256i;

    // Transform 1
    w0 = read8(input.add(0));
    w1 = read8(input.add(4));
    w2 = read8(input.add(8));
    w3 = read8(input.add(12));
    w4 = read8(input.add(16));
    w5 = read8(input.add(20));
    w6 = read8(input.add(24));
    w7 = read8(input.add(28));
    w8 = read8(input.add(32));
    w9 = read8(input.add(36));
    w10 = read8(input.add(40));
    w11 = read8(input.add(44));
    w12 = read8(input.add(48));
    w13 = read8(input.add(52));
    w14 = read8(input.add(56));
    w15 = read8(input.add(60));
    round(a, b, c, &mut d, e, f, g, &mut h, add(k(K32[0]), w0));
    round(h, a, b, &mut c, d, e, f, &mut g, add(k(K32[1]), w1));
    round(g, h, a, &mut b, c, d, e, &mut f, add(k(K32[2]), w2));
    round(f, g, h, &mut a, b, c, d, &mut e, add(k(K32[3]), w3));
    round(e, f, g, &mut h, a, b, c, &mut d, add(k(K32[4]), w4));
    round(d, e, f, &mut g, h, a, b, &mut c, add(k(K32[5]), w5));
    round(c, d, e, &mut f, g, h, a, &mut b, add(k(K32[6]), w6));
    round(b, c, d, &mut e, f, g, h, &mut a, add(k(K32[7]), w7));
    round(a, b, c, &mut d, e, f, g, &mut h, add(k(K32[8]), w8));
    round(h, a, b, &mut c, d, e, f, &mut g, add(k(K32[9]), w9));
    round(g, h, a, &mut b, c, d, e, &mut f, add(k(K32[10]), w10));
    round(f, g, h, &mut a, b, c, d, &mut e, add(k(K32[11]), w11));
    round(e, f, g, &mut h, a, b, c, &mut d, add(k(K32[12]), w12));
    round(d, e, f, &mut g, h, a, b, &mut c, add(k(K32[13]), w13));
    round(c, d, e, &mut f, g, h, a, &mut b, add(k(K32[14]), w14));
    round(b, c, d, &mut e, f, g, h, &mut a, add(k(K32[15]), w15));

    round(a, b, c, &mut d, e, f, g, &mut h, add(k(K32[16] as i32), inc4(&mut w0, sigma1(w14), w9, sigma0(w1))));
    round(h, a, b, &mut c, d, e, f, &mut g, add(k(K32[17] as i32), inc4(&mut w1, sigma1(w15), w10, sigma0(w2))));
    round(g, h, a, &mut b, c, d, e, &mut f, add(k(K32[18] as i32), inc4(&mut w2, sigma1(w0), w11, sigma0(w3))));
    round(f, g, h, &mut a, b, c, d, &mut e, add(k(K32[19] as i32), inc4(&mut w3, sigma1(w1), w12, sigma0(w4))));
    round(e, f, g, &mut h, a, b, c, &mut d, add(k(K32[20] as i32), inc4(&mut w4, sigma1(w2), w13, sigma0(w5))));
    round(d, e, f, &mut g, h, a, b, &mut c, add(k(K32[21] as i32), inc4(&mut w5, sigma1(w3), w14, sigma0(w6))));
    round(c, d, e, &mut f, g, h, a, &mut b, add(k(K32[22] as i32), inc4(&mut w6, sigma1(w4), w15, sigma0(w7))));
    round(b, c, d, &mut e, f, g, h, &mut a, add(k(K32[23] as i32), inc4(&mut w7, sigma1(w5), w0, sigma0(w8))));
    round(a, b, c, &mut d, e, f, g, &mut h, add(k(K32[24] as i32), inc4(&mut w8, sigma1(w6), w1, sigma0(w9))));
    round(h, a, b, &mut c, d, e, f, &mut g, add(k(K32[25] as i32), inc4(&mut w9, sigma1(w7), w2, sigma0(w10))));
    round(g, h, a, &mut b, c, d, e, &mut f, add(k(K32[26] as i32), inc4(&mut w10, sigma1(w8), w3, sigma0(w11))));
    round(f, g, h, &mut a, b, c, d, &mut e, add(k(K32[27] as i32), inc4(&mut w11, sigma1(w9), w4, sigma0(w12))));
    round(e, f, g, &mut h, a, b, c, &mut d, add(k(K32[28] as i32), inc4(&mut w12, sigma1(w10), w5, sigma0(w13))));
    round(d, e, f, &mut g, h, a, b, &mut c, add(k(K32[29] as i32), inc4(&mut w13, sigma1(w11), w6, sigma0(w14))));
    round(c, d, e, &mut f, g, h, a, &mut b, add(k(K32[30] as i32), inc4(&mut w14, sigma1(w12), w7, sigma0(w15))));
    round(b, c, d, &mut e, f, g, h, &mut a, add(k(K32[31] as i32), inc4(&mut w15, sigma1(w13), w8, sigma0(w0))));
    round(a, b, c, &mut d, e, f, g, &mut h, add(k(K32[32] as i32), inc4(&mut w0, sigma1(w14), w9, sigma0(w1))));
    round(h, a, b, &mut c, d, e, f, &mut g, add(k(K32[33] as i32), inc4(&mut w1, sigma1(w15), w10, sigma0(w2))));
    round(g, h, a, &mut b, c, d, e, &mut f, add(k(K32[34] as i32), inc4(&mut w2, sigma1(w0), w11, sigma0(w3))));
    round(f, g, h, &mut a, b, c, d, &mut e, add(k(K32[35] as i32), inc4(&mut w3, sigma1(w1), w12, sigma0(w4))));
    round(e, f, g, &mut h, a, b, c, &mut d, add(k(K32[36] as i32), inc4(&mut w4, sigma1(w2), w13, sigma0(w5))));
    round(d, e, f, &mut g, h, a, b, &mut c, add(k(K32[37] as i32), inc4(&mut w5, sigma1(w3), w14, sigma0(w6))));
    round(c, d, e, &mut f, g, h, a, &mut b, add(k(K32[38] as i32), inc4(&mut w6, sigma1(w4), w15, sigma0(w7))));
    round(b, c, d, &mut e, f, g, h, &mut a, add(k(K32[39] as i32), inc4(&mut w7, sigma1(w5), w0, sigma0(w8))));
    round(a, b, c, &mut d, e, f, g, &mut h, add(k(K32[40] as i32), inc4(&mut w8, sigma1(w6), w1, sigma0(w9))));
    round(h, a, b, &mut c, d, e, f, &mut g, add(k(K32[41] as i32), inc4(&mut w9, sigma1(w7), w2, sigma0(w10))));
    round(g, h, a, &mut b, c, d, e, &mut f, add(k(K32[42] as i32), inc4(&mut w10, sigma1(w8), w3, sigma0(w11))));
    round(f, g, h, &mut a, b, c, d, &mut e, add(k(K32[43] as i32), inc4(&mut w11, sigma1(w9), w4, sigma0(w12))));
    round(e, f, g, &mut h, a, b, c, &mut d, add(k(K32[44] as i32), inc4(&mut w12, sigma1(w10), w5, sigma0(w13))));
    round(d, e, f, &mut g, h, a, b, &mut c, add(k(K32[45] as i32), inc4(&mut w13, sigma1(w11), w6, sigma0(w14))));
    round(c, d, e, &mut f, g, h, a, &mut b, add(k(K32[46] as i32), inc4(&mut w14, sigma1(w12), w7, sigma0(w15))));
    round(b, c, d, &mut e, f, g, h, &mut a, add(k(K32[47] as i32), inc4(&mut w15, sigma1(w13), w8, sigma0(w0))));
    round(a, b, c, &mut d, e, f, g, &mut h, add(k(K32[48] as i32), inc4(&mut w0, sigma1(w14), w9, sigma0(w1))));
    round(h, a, b, &mut c, d, e, f, &mut g, add(k(K32[49] as i32), inc4(&mut w1, sigma1(w15), w10, sigma0(w2))));
    round(g, h, a, &mut b, c, d, e, &mut f, add(k(K32[50] as i32), inc4(&mut w2, sigma1(w0), w11, sigma0(w3))));
    round(f, g, h, &mut a, b, c, d, &mut e, add(k(K32[51] as i32), inc4(&mut w3, sigma1(w1), w12, sigma0(w4))));
    round(e, f, g, &mut h, a, b, c, &mut d, add(k(K32[52] as i32), inc4(&mut w4, sigma1(w2), w13, sigma0(w5))));
    round(d, e, f, &mut g, h, a, b, &mut c, add(k(K32[53] as i32), inc4(&mut w5, sigma1(w3), w14, sigma0(w6))));
    round(c, d, e, &mut f, g, h, a, &mut b, add(k(K32[54] as i32), inc4(&mut w6, sigma1(w4), w15, sigma0(w7))));
    round(b, c, d, &mut e, f, g, h, &mut a, add(k(K32[55] as i32), inc4(&mut w7, sigma1(w5), w0, sigma0(w8))));
    round(a, b, c, &mut d, e, f, g, &mut h, add(k(K32[56] as i32), inc4(&mut w8, sigma1(w6), w1, sigma0(w9))));
    round(h, a, b, &mut c, d, e, f, &mut g, add(k(K32[57] as i32), inc4(&mut w9, sigma1(w7), w2, sigma0(w10))));
    round(g, h, a, &mut b, c, d, e, &mut f, add(k(K32[58] as i32), inc4(&mut w10, sigma1(w8), w3, sigma0(w11))));
    round(f, g, h, &mut a, b, c, d, &mut e, add(k(K32[59] as i32), inc4(&mut w11, sigma1(w9), w4, sigma0(w12))));
    round(e, f, g, &mut h, a, b, c, &mut d, add(k(K32[60] as i32), inc4(&mut w12, sigma1(w10), w5, sigma0(w13))));
    round(d, e, f, &mut g, h, a, b, &mut c, add(k(K32[61] as i32), inc4(&mut w13, sigma1(w11), w6, sigma0(w14))));
    round(c, d, e, &mut f, g, h, a, &mut b, add(k(K32[62] as i32), inc4(&mut w14, sigma1(w12), w7, sigma0(w15))));
    round(b, c, d, &mut e, f, g, h, &mut a, add(k(K32[63] as i32), inc4(&mut w15, sigma1(w13), w8, sigma0(w0))));

    a = add(a, k(H256[0] as i32));
    b = add(b, k(H256[1] as i32));
    c = add(c, k(H256[2] as i32));
    d = add(d, k(H256[3] as i32));
    e = add(e, k(H256[4] as i32));
    f = add(f, k(H256[5] as i32));
    g = add(g, k(H256[6] as i32));
    h = add(h, k(H256[7] as i32));

    *digest = [a, b, c, d, e, f, g, h];
}

//#[start]
#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        let mut input: [u8; 576] = [0; 576];
        let mut out: [u8; 32] = [0; 32];

        for i in 0..512 {
            input[i] = 0x61;
        }
        input[512] = 0x80;
        write_be64(input.as_mut_ptr().add(input.len() - 8), 512 * 8);

        println!("Input (8way):\n{:02X?}", input);

        let mut digest = digest_init();
        for i in 0..input.len() / 64 / 8 {
            transform_8way(&mut digest, input.as_ptr().add(i * 64 * 8));
        }
        digest_finish(&mut out, digest);

        println!("Result (8way):\n{:02X?}", out);

        assert_eq!(out,
                   [0xcd, 0xc7, 0x6e, 0x5c, 0x99, 0x14, 0xfb, 0x92, 0x81, 0xa1, 0xc7, 0xe2, 0x84, 0xd7, 0x3e, 0x67,
                    0xf1, 0x80, 0x9a, 0x48, 0xa4, 0x97, 0x20, 0x0e, 0x04, 0x6d, 0x39, 0xcc, 0xc7, 0x11, 0x2c, 0xd0]);
    }

    0
}
