#[link(name = "test", kind = "static")]
extern "C" {
    fn hello_world();
}
pub fn hello_by_cpp() {
    unsafe {
        hello_world();
    }
}
