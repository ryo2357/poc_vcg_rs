#[link(name = "meshlab", kind = "static")]
extern "C" {
    fn print_tetrahedron();
}

pub fn print_tetrahedron_by_cpp() {
    unsafe { print_tetrahedron() };
}
