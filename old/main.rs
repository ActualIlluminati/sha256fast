#![no_main]

const STATE_LEN: usize = 8;

const H256: [u32; STATE_LEN] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c,
                                0x1f83d9ab, 0x5be0cd19];
const K32: [u32; 64] =
    [0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
     0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
     0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
     0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
     0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
     0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
     0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
     0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2];

#[inline(always)]
pub unsafe fn k(x: u32) -> u32 { x }
#[inline(always)]
pub unsafe fn add(x: u32, y: u32) -> u32 { x.wrapping_add(y) }
#[inline(always)]
pub unsafe fn add3(x: u32, y: u32, z: u32) -> u32 { add(add(x, y), z) }
#[inline(always)]
pub unsafe fn add4(x: u32, y: u32, z: u32, w: u32) -> u32 { add(add(x, y), add(z, w)) }
#[inline(always)]
pub unsafe fn add5(x: u32, y: u32, z: u32, w: u32, v: u32) -> u32 { add(add3(x, y, z), add(w, v)) }
#[inline(always)]
pub unsafe fn inc(x: &mut u32, y: u32) -> u32 {
    *x = add(*x, y);
    *x
}
#[inline(always)]
pub unsafe fn inc3(x: &mut u32, y: u32, z: u32) -> u32 {
    *x = add3(*x, y, z);
    *x
}
#[inline(always)]
pub unsafe fn inc4(x: &mut u32, y: u32, z: u32, w: u32) -> u32 {
    *x = add4(*x, y, z, w);
    *x
}
#[inline(always)]
pub unsafe fn xor(x: u32, y: u32) -> u32 { x ^ y }
#[inline(always)]
pub unsafe fn xor3(x: u32, y: u32, z: u32) -> u32 { xor(xor(x, y), z) }
#[inline(always)]
pub unsafe fn or(x: u32, y: u32) -> u32 { x | y }
#[no_mangle]
#[inline(always)]
pub unsafe fn and(x: u32, y: u32) -> u32 { x & y }
#[inline(always)]
pub unsafe fn shr(x: u32, n: u32) -> u32 { x >> n }
#[inline(always)]
pub unsafe fn rotr(x: u32, n: u32) -> u32 { x.rotate_right(n) }
#[inline(always)]
pub unsafe fn shl(x: u32, n: u32) -> u32 { x << n }
#[inline(always)]
pub unsafe fn rotl(x: u32, n: u32) -> u32 { x.rotate_left(n) }

#[inline(always)]
pub unsafe fn ch(x: u32, y: u32, z: u32) -> u32 { xor(z, and(x, xor(y, z))) }
#[inline(always)]
pub unsafe fn maj(x: u32, y: u32, z: u32) -> u32 { xor3(and(x, y), and(x, z), and(y, z)) }
#[inline(always)]
pub unsafe fn big_sigma0(x: u32) -> u32 { xor3(rotr(x, 2), rotr(x, 13), rotr(x, 22)) }
#[inline(always)]
pub unsafe fn big_sigma1(x: u32) -> u32 { xor3(rotr(x, 6), rotr(x, 11), rotr(x, 25)) }
#[inline(always)]
pub unsafe fn sigma0(x: u32) -> u32 { xor3(rotr(x, 7), rotr(x, 18), shr(x, 3)) }
#[inline(always)]
pub unsafe fn sigma1(x: u32) -> u32 { xor3(rotr(x, 17), rotr(x, 19), shr(x, 10)) }

#[inline(always)]
pub unsafe fn round(a: u32,
                    b: u32,
                    c: u32,
                    d: &mut u32,
                    e: u32,
                    f: u32,
                    g: u32,
                    h: &mut u32,
                    k: u32,
                    w: u32) {
    let t1: u32 = add5(*h, big_sigma1(e), ch(e, f, g), k, w);
    let t2: u32 = add(big_sigma0(a), maj(a, b, c));
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

pub unsafe fn digest_init() -> [u32; STATE_LEN] { H256.clone() }
pub unsafe fn digest_finish(out: &mut [u8; 32], digest: [u32; STATE_LEN]) {
    for i in 0..digest.len() {
        write_be32(out.as_mut_ptr().add(4 * i), digest[i]);
    }
}

#[inline(always)]
pub unsafe fn transform_1way(digest: &mut [u32; STATE_LEN], input: *const u8) {
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *digest;

    let mut w: [u32; 64] = [0; 64];

    for i in 0..16 {
        w[i] = read_be32(input.add(i * 4));
    }
    for i in 16..64 {
        w[i] = add4(w[i - 16], sigma1(w[i - 2]), w[i - 7], sigma0(w[i - 15]));
    }
    // println!("Chunk W (1way):\n{:02X?}", w);

    for i in 0..64 {
        round(a, b, c, &mut d, e, f, g, &mut h, K32[i], w[i]);

        let h_tmp = h;
        h = g;
        g = f;
        f = e;
        e = d;
        d = c;
        c = b;
        b = a;
        a = h_tmp;
    }

    a = add(a, digest[0]);
    b = add(b, digest[1]);
    c = add(c, digest[2]);
    d = add(d, digest[3]);
    e = add(e, digest[4]);
    f = add(f, digest[5]);
    g = add(g, digest[6]);
    h = add(h, digest[7]);

    *digest = [a, b, c, d, e, f, g, h];
}

//#[start]
#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        let mut input: [u8; 64] = [0; 64];
        let mut out: [u8; 32] = [0; 32];

        input[0] = 0x61;
        input[1] = 0x62;
        input[2] = 0x63;
        input[3] = 0x80;
        write_be64(input.as_mut_ptr().add(56), 3 * 8);

        println!("Input (1way):\n{:02X?}", input);

        let mut digest = digest_init();
        transform_1way(&mut digest, input.as_ptr());
        digest_finish(&mut out, digest);

        println!("Result (1way):\n{:02X?}", out);

        assert_eq!(out,
                   [0xBA, 0x78, 0x16, 0xBF, 0x8F, 0x01, 0xCF, 0xEA, 0x41, 0x41, 0x40, 0xDE, 0x5D, 0xAE,
                    0x22, 0x23, 0xB0, 0x03, 0x61, 0xA3, 0x96, 0x17, 0x7A, 0x9C, 0xB4, 0x10, 0xFF, 0x61,
                    0xF2, 0x00, 0x15, 0xAD]);
    }

    unsafe {
        let mut input: [u8; 1000064] = [0; 1000064];
        let mut out: [u8; 32] = [0; 32];

        for i in 0..1000000 {
            input[i] = 0x61;
        }
        input[1000000] = 0x80;
        write_be64(input.as_mut_ptr().add(1000064 - 8), 1000000 * 8);

        let mut digest = digest_init();
        for i in 0..input.len() / 64 {
            transform_1way(&mut digest, input.as_ptr().add(i * 64));
        }
        digest_finish(&mut out, digest);

        println!("Result (1way):\n{:02X?}", out);

        assert_eq!(out,
                   [0xcd, 0xc7, 0x6e, 0x5c, 0x99, 0x14, 0xfb, 0x92, 0x81, 0xa1, 0xc7, 0xe2, 0x84, 0xd7,
                    0x3e, 0x67, 0xf1, 0x80, 0x9a, 0x48, 0xa4, 0x97, 0x20, 0x0e, 0x04, 0x6d, 0x39, 0xcc,
                    0xc7, 0x11, 0x2c, 0xd0]);
    }

    0
}
