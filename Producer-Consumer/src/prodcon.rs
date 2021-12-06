extern "C" {
    pub fn Trans(n: ::std::os::raw::c_int);
    pub fn Sleep(n: ::std::os::raw::c_int);
}

pub fn main() {
    unsafe {
        Trans(1);
        Sleep(1);
    }
}
