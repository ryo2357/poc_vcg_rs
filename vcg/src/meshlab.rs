#[link(name = "meshlab", kind = "static")]
#[allow(improper_ctypes)]
extern "C" {
    fn print_tetrahedron();
}

pub fn print_tetrahedron_by_cpp() {
    unsafe { print_tetrahedron() };
}
