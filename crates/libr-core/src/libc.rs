#[repr(transparent)]
#[lang = "CStr"]
pub struct CStr {
    inner: [i8],
}

impl CStr {
    pub fn as_ptr(&self) -> *const i8 {
        &self.inner as *const [i8] as *const i8
    }
}


#[cfg(target_os = "linux")]
#[link(name = "c")]
unsafe extern "C" {
    pub unsafe fn printf(format: *const i8, ...) -> i32;
}