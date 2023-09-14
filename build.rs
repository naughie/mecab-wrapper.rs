#[cfg(feature = "cmecab")]
fn main() {
    println!("cargo:rerun-if-changed=lib/cmecab.cpp");
    cc::Build::new()
        .cpp(true)
        .file("lib/cmecab.cpp")
        .cpp_link_stdlib("stdc++")
        .compile("libcmecab.a");
    println!("cargo:rustc-link-lib=mecab");
}

#[cfg(not(feature = "cmecab"))]
fn main() {}
