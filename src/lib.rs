#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "core")]
extern crate core;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

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
pub type CChar = ::core::ffi::c_char;

#[no_mangle]
pub extern "C" fn abs(i: CInt) -> CInt {
    i.abs()
}

#[cfg(not(target_env = "msvc"))]
#[cfg(feature = "alloc")]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[cfg(feature = "alloc")]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[no_mangle]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[no_mangle]
pub extern "C" fn isupper(c: CInt) -> bool {
    (c as u32 - b'A' as u32) < 26
}

#[cfg(feature = "alloc")]
use alloc::{string::String, vec, vec::Vec, fmt};

#[cfg(feature = "alloc")]
use libc_print::std_name::{println, eprintln, dbg};

#[no_mangle]
#[cfg(feature = "alloc")]
pub extern "C" fn test_vec() {
    let mut v = vec![1,2,3];
    let res = v.pop();
}

#[no_mangle]
#[cfg(feature = "alloc")]
pub extern "C" fn format()  {
    println!("Hi");
}

#[no_mangle]
pub unsafe extern "C" fn strlen(mut s: *const CChar) -> usize {
    let mut length = 0;
    while *s != 0 {
        s = s.offset(1);
        length += 1;
    }
    length
}

#[no_mangle]
pub unsafe extern "C" fn print(print_str: *const CChar) {
    write(1usize, print_str as usize, unsafe { strlen(print_str) });
}

pub mod syscall;
pub use syscall::*;

#[cfg(not(feature = "std"))]
use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(feature = "std"))]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
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

    #[test]
    fn syscalls() {
        let string = "Hello World\n";

        let ptr = string.as_ptr() as usize;
        let len = string.len();
        write(1usize, ptr, len);
    }
}
