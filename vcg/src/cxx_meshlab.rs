#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vcg/c_src/cxx_test/meshlab.h");

        fn print_tetrahedron();
    }
}

pub fn test() {
    ffi::print_tetrahedron();
}
