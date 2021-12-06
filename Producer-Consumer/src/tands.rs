extern "C" {
    pub fn Trans(n: ::std::os::raw::c_int);
    pub fn Sleep(n: ::std::os::raw::c_int);
}

pub fn trans(n: i32) {
    unsafe { Trans(n) }
}

pub fn sleep(n: i32) {
    unsafe { Sleep(n) }
}
