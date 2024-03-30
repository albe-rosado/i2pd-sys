#[cxx::bridge(namespace = "i2p::api")]
mod i2pd_ffi {
    unsafe extern "C++" {
        include!("i2pd-sys/i2pd/libi2pd/api.h");
        #[rust_name = "init_i2p"]
        unsafe fn InitI2P(argc: i32, argv: *mut *mut c_char, appName: *const c_char);
        #[rust_name = "start_i2p"]
        fn StartI2P();
        #[rust_name = "stop_i2p"]
        fn StopI2P();
        #[rust_name = "terminate_i2p"]
        fn TerminateI2P();
    }
}

pub mod i2pd {
    use crate::i2pd_ffi;
    use std::ffi::CString;

    pub fn initialize(app_name: &str) {
        let app_name = CString::new(app_name).unwrap();
        unsafe {
            i2pd_ffi::init_i2p(1, core::ptr::null_mut(), app_name.as_ptr());
        }
    }
    pub fn start() {
        i2pd_ffi::start_i2p()
    }
    pub fn stop() {
        i2pd_ffi::stop_i2p()
    }
    pub fn terminate() {
        i2pd_ffi::terminate_i2p()
    }
}

#[cfg(test)]
mod tests {
    use crate::i2pd_ffi;
    use std::{ffi::CString, thread, time::Duration};

    #[test]
    fn it_works() {
        let app_name = CString::new("ffi_app").unwrap();
        unsafe {
            i2pd_ffi::init_i2p(1, core::ptr::null_mut(), app_name.as_ptr());
        }
        i2pd_ffi::start_i2p();
        thread::sleep(Duration::from_secs(3));
        i2pd_ffi::stop_i2p();
        assert!(true);
    }
}
