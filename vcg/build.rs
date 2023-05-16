fn main() {
    println!("cargo clean -p vcg");

    cc::Build::new().file("c_src/c_test.c").compile("c_test");
}
