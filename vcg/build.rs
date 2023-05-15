fn main() {
    cc::Build::new().file("c_src/c_test.c").compile("c_test");
}
