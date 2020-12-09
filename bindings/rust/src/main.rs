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

    {
        let data = b"abc";
        println!("Data: {}", String::from_utf8(data.to_vec()).unwrap());
        let hashed = Sha256::digest(data);
        unsafe {
            let mut sctx: sha256fast::Sha256State = Default::default();
            let mut out = [0u8; 32];
            sha256fast::sha256_update(&mut sctx, data.as_ptr(), data.len() as u32);
            sha256fast::sha256_final(&mut sctx, out.as_mut_ptr());
            println!("Hash: {:02X?}", out);
        }
        println!("Hash: {:02X?}", hashed);
        println!("Expected: ba7816bf 8f01cfea 414140de 5dae2223 b00361a3 96177a9c b410ff61 f20015ad");
    }

    {
        let data = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
        println!("Data: {}", String::from_utf8(data.to_vec()).unwrap());
        let hashed = Sha256::digest(data);
        unsafe {
            let mut sctx: sha256fast::Sha256State = Default::default();
            let mut out = [0u8; 32];
            sha256fast::sha256_update(&mut sctx, data.as_ptr(), data.len() as u32);
            sha256fast::sha256_final(&mut sctx, out.as_mut_ptr());
            println!("Hash: {:02X?}", out);
        }
        println!("Hash: {:02X?}", hashed);
        println!("Expected: cf5b16a7 78af8380 036ce59e 7b049237 0b249b11 e8f07a51 afac4503 7afee9d1");
    }
}
