use std::mem;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

#[no_mangle]
pub extern fn sum(s: *mut c_char) -> u32 {
    let s = unsafe {
        CStr::from_ptr(s)
    };

    let s = s.to_str().unwrap();

    let mut sum = 0;
    let bytes = s.as_bytes();

    for (idx, &c) in bytes.iter().enumerate() {
        let second_index = (idx + 1) % bytes.len();

        // convert bytes to digits
        let num = c - 48;
        let second_num = bytes[second_index] - 48;

        if num == second_num {
            sum += num as u32;
        }
    }

    sum
}

// magic wasm shenanigans

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf); // This is JS' responsibility now
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[cfg(test)]
mod tests {
    // these are going to leak but it's tests so I don't care
    use std::ffi::CString;
    use super::sum;

    #[test]
    fn first() {
        let s = CString::new("1122").unwrap().into_raw();

        assert_eq!(sum(s), 3);
    }

    #[test]
    fn second() {
        let s = CString::new("1111").unwrap().into_raw();

        assert_eq!(sum(s), 4);
    }
    
    #[test]
    fn third() {
        let s = CString::new("1234").unwrap().into_raw();

        assert_eq!(sum(s), 0);
    }
    
    #[test]
    fn fourth() {
        let s = CString::new("91212129").unwrap().into_raw();

        assert_eq!(sum(s), 9);
    }
}
