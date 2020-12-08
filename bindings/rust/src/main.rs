use sha256fast::{Digest, Sha256};

fn main() {
    println!("Hello, world!");

    let data = b"abc";
    // let mut out: [u8; 32] = [0; 32];
    for _ in 0..10_000_000 {
        let _hashed = Sha256::digest(data);
        // unsafe {
        // let mut sctx: Sha256 = Default::default();
        // sha256_update(&mut sctx, data.as_ptr(), 3);
        // sha256_final(&mut sctx, out.as_mut_ptr());
        // }
    }
    let out = Sha256::digest(data);

    println!("Out: {:02X?}", out);
}
