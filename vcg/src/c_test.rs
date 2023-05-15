type LONG = std::os::raw::c_int;
// OSによるが基本的にi32

#[link(name = "c_test", kind = "static")]
#[allow(improper_ctypes)]
extern "C" {
    fn factorial(n: LONG) -> LONG;
}

pub fn factorial_by_clang(n: usize) -> usize {
    let c_n: LONG = n as LONG;

    let c_return = unsafe { factorial(c_n) };

    return c_return as usize;
}
