use std::ffi;

use crate::error::DemangleError;

extern "C" {
    fn swift_demangle(
        mangledName: *const u8,
        mangledNameLength: usize,
        outputBuffer: *mut u8,
        outputBufferSize: *mut usize,
        flags: u32,
    ) -> *const i8;
}



pub fn demangle(s: &str) -> Result<&str, DemangleError> {
    unsafe {
        let demangled = swift_demangle(
            s.as_ptr(),
            s.len(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
        );
        if demangled.is_null() {
            Err(DemangleError::Null)
        } else {
            ffi::CStr::from_ptr(demangled)
                .to_str()
                .map_err(DemangleError::Utf8Error)
        }
    }
}

#[test]
fn test_demangle() {
    assert_eq!(demangle("$sSa").unwrap(), "Swift.Array");
}
