fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/torii_client.cpp")
        .flag_if_supported("-Wall")
        .include("include")
        .flag_if_supported("-std=c++14")
        .compile("torii_client");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=include/torii_client.h");
    println!("cargo:rerun-if-changed=src/torii_client.cpp");
}
