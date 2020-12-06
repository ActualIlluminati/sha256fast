use sha256fast::*;

fn main() {
    println!("Hello, world!");

    let data = "abc";
    let mut out: [u8; 32] = [0; 32];
    for i in 0..10_000_000 {
        unsafe {
            let mut sctx = sha256_new();
            sha256_update(&mut sctx, data.as_ptr(), 3);
            sha256_final(&mut sctx, out.as_mut_ptr());
        }
    }

    println!("Out: {:02X?}", out);
}
