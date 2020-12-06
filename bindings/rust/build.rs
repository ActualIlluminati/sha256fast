#![allow(unused_imports)]

use std::path::Path;

fn main() {
    /*
     * Use pre-built libblst.a if there is one. This is primarily
     * for trouble-shooting purposes. Idea is that libblst.a can be
     * compiled with flags independent from cargo defaults, e.g.
     * '../../build.sh -O1 ...'.
     */
    if Path::new("libsha256fast.a").exists() {
        println!("cargo:rustc-link-search=.");
        println!("cargo:rustc-link-lib=sha256fast");
        return;
    }
}
