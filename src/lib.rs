use std::ffi::c_void;
use libc::*;

#[link(name = "mylib")]
extern "C"
{
    pub fn my_malloc(size: size_t) -> *mut c_void;
    pub fn my_free(ptr: *mut c_void);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leak_test()
    {
        unsafe
        {
            let _ptr = my_malloc(4096);
        }
    }
}
