#![allow(unused_imports)]

extern crate cc;

use std::env;
use std::path::{Path, PathBuf};

#[cfg(all(target_pointer_width = "64", not(target_env = "msvc")))]
fn assembly(file_vec: &mut Vec<PathBuf>, base_dir: &Path) {
    // file_vec.push(base_dir.join("sha256-ssse3-asm.S"))
    file_vec.push(base_dir.join("sha256-avx2-asm.S"))
}

fn main() {
    /*
     * Use pre-built libsha256fast.a if there is one. This is primarily
     * for trouble-shooting purposes. Idea is that libsha256fast.a can be
     * compiled with flags independent from cargo defaults, e.g.
     * '../../build.sh -O1 ...'.
     */
    if Path::new("libsha256fast.a").exists() {
        println!("cargo:rustc-link-search=.");
        println!("cargo:rustc-link-lib=sha256fast");
        return;
    }

    let mut file_vec = Vec::new();

    let sha256fast_base_dir = match env::var("SHA256FAST_SRC_DIR") {
        Ok(val) => PathBuf::from(val),
        Err(_) => {
            let local_sha256fast = PathBuf::from("sha256fast");
            if local_sha256fast.exists() {
                local_sha256fast
            } else {
                // Reach out to ../.., which is the root of the sha256fast repo.
                // Use an absolute path to avoid issues with relative paths
                // being treated as strings by `cc` and getting concatenated
                // in ways that reach out of the OUT_DIR.
                env::current_dir()
                    .expect("can't access current directory")
                    .parent()
                    .and_then(|dir| dir.parent())
                    .expect("can't access parent of parent of current directory")
                    .into()
            }
        }
    };
    println!("Using sha256fast source directory {}", sha256fast_base_dir.display());

    let c_src_dir = sha256fast_base_dir.join("src");

    // file_vec.push(c_src_dir.join("sha256-generic.cxx"));
    // file_vec.push(c_src_dir.join("sha256-ssse3.cxx"));
    file_vec.push(c_src_dir.join("sha256-avx2.cxx"));
    #[cfg(all(target_pointer_width = "64"))]
    assembly(&mut file_vec, &c_src_dir);

    // Set CC environment variable to choose alternative C compiler.
    // Optimization level depends on whether or not --release is passed
    // or implied.
    let mut cc = cc::Build::new();
    cc.cpp(true);

    if !cfg!(debug_assertions) {
        cc.opt_level(2);
    }
    cc.files(&file_vec).compile("libsha256fast.a");
}
