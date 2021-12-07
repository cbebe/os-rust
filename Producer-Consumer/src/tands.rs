extern "C" {
    pub fn Trans(n: ::std::os::raw::c_int);
    pub fn Sleep(n: ::std::os::raw::c_int);
}

#[inline]
pub fn trans(n: u32) {
    unsafe { Trans(n as i32) }
}

#[inline]
pub fn sleep(n: u32) {
    unsafe { Sleep(n as i32) }
}
