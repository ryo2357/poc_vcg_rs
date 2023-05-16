fn main() {
    println!("cargo clean -p vcg");

    // cc::Build::new().file("c_src/c_test.c").compile("c_test");

    // cc::Build::new()
    //     .cpp(true)
    //     .file("c_src/meshlab.cpp")
    //     .include("c_src/meshlab.h")
    //     .compile("meshlab.a");

    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .file("./c_src/cpp_test/test.cpp")
        .compile("libtest.a");
}
