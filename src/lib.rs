#![no_std]
/// `long long int`
pub type CLongLong = ::core::ffi::c_longlong;

/// `unsigned long long int`
pub type CULongLong = ::core::ffi::c_ulonglong;

/// `long int`
pub type CLong = ::core::ffi::c_long;

/// `unsigned long int`
pub type CULong = ::core::ffi::c_ulong;

/// `int`
pub type CInt = ::core::ffi::c_int;

/// `unsigned int`
pub type CUInt = ::core::ffi::c_uint;

/// 8-bit Char
pub type CChar = u8;

#[no_mangle]
pub extern "C" fn abs(i: CInt) -> CInt {
    i.abs()
}

#[no_mangle]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[no_mangle]
pub extern "C" fn isupper(c: CInt) -> bool {
    (c as u32 - b'A' as u32) < 26
}

/// # Safety
/// This should be safe
#[no_mangle]
pub unsafe extern "C" fn strlen(mut s: *const CChar) -> usize {
    let mut length = 0;
    while *s != 0 {
        s = s.offset(1);
        length += 1;
    }
    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(abs(-2), 2);
    }

    #[test]
    fn test_addition() {
        assert_eq!(add(3, 5), 8);
    }

    #[test]
    fn test_isupper() {
        assert!(isupper('A' as CInt));
    }

    #[test]
    fn test_isupper_a() {
        assert!(!isupper('a' as CInt));
    }

    #[test]
    fn test_strlen() {
        assert_eq!(unsafe { strlen(b"\0" as *const CChar) }, 0);
    }
}
