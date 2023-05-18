fn main() {
    println!("cargo clean -p vcg");

    cc::Build::new().file("c_src/c_test.c").compile("c_test");

    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .file("./c_src/cpp_test/test.cpp")
        .compile("libtest.a");

    // 理由は分からないがビルドできない
    // cc::Build::new()
    //     .cpp(true)
    //     .warnings(true)
    //     .flag("-Wall")
    //     .flag("-Wextra")
    //     .flag("-v")
    //     .flag("-g")
    //     .file("./c_src/cpp_test/meshlab.cpp")
    //     .compile("libmeshlab.a");

    cxx_build::bridge("src/cxx_tutorial.rs")
        .file("c_src/cxx_test/blobstore.cc")
        .compile("cxx-demo");
    // こっちはコンパイル通らない
    cxx_build::bridge("src/cxx_meshlab.rs")
        .file("c_src/cxx_test/meshlab.cc")
        .file("c_src/vcglib/wrap/ply/plylib.cpp")
        .include("c_src/vcglib")
        .include("c_src/eigen")
        .include("c_src/meshlab/src")
        .flag("-EHsc")
        .compile("cxx-messhlab");

    // 理由は分からないがビルドできない
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        // .flag("-Wall")
        // .flag("-Wextra")
        // .flag("-v")
        // .flag("-g")
        .file("./c_src/cpp_test/meshlab.cpp")
        .include("c_src/vcglib")
        .include("c_src/eigen")
        .include("c_src/meshlab/src")
        .compile("libmeshlab.a");
}
