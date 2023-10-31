fn main() {
    cxx_build::bridge("src/main.rs")
        .flag_if_supported("-std=c++14")
        .compile("cxxbridge-dojo");

    println!("cargo:rerun-if-changed=src/main.rs");
}
