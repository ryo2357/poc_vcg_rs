use mylogger::{debug, error, info, init, warn};

fn main() {
    init();
    let a = vcg::c_test::factorial_by_clang(6);
    info!("a: {:?}", a);
}
