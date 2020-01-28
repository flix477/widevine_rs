fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .file("cppbridge/api.cpp")
        .file("cppbridge/implementation/host.cpp")
        .file("cppbridge/implementation/buffer.cpp")
        .file("cppbridge/implementation/decrypted_block.cpp")
        .include("cppbridge")
        .compile("cppbridge");
}
