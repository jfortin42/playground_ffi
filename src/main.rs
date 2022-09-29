 use playground_ffi::*;
fn main()
{
    unsafe
    {
        let _ptr = my_malloc(4096);
    }
}

