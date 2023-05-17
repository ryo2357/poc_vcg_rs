use mylogger::{debug, error, info, init, warn};

fn main() {
    init();
    let a = vcg::c_test::factorial_by_clang(6);
    info!("a: {:?}", a);

    vcg::cpp_test::hello_by_cpp();

    vcg::cxx_tutorial::test();
    // vcg::meshlab::print_tetrahedron_by_cpp();
}
