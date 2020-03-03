use std::ffi::{CStr,CString};
use std::str;
use std::mem;
use std::os::raw::c_char;

use strsim::*;

fn to_string(s: *const c_char) -> String {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();
    r_str.to_string()
}


#[no_mangle]
#[allow(non_snake_case)]
pub fn levenshtein(a: *const c_char, b: *const c_char) -> usize {
    let a = to_string(a);
    let b = to_string(b);
    strsim::levenshtein(&(a.as_str()), &(b.as_str()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
