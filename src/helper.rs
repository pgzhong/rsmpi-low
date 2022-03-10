use std::mem::size_of;
use std::os::raw::c_int;

pub fn check_buf_size<T>(buf: &[T], count: c_int){
    let sizeof_buf = size_of::<T>() * buf.len();
    assert!(sizeof_buf >= count.try_into().unwrap());
}
