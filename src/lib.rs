#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(vec_into_raw_parts)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "core")]
extern crate core;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(not(any(feature = "std", test)))]
use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(any(feature = "std", test)))]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    core::intrinsics::abort();
}

#[no_mangle]
#[lang = "eh_personality"]
#[cfg(not(any(feature = "std", test)))]
pub extern "C" fn eh_personality() {}

#[cfg(not(target_env = "msvc"))]
#[cfg(not(any(feature = "std", test)))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[cfg(not(any(feature = "std", test)))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

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

// Ref CString
pub type CStr = ::core::ffi::CStr;

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

#[cfg(feature = "alloc")]
use alloc::{fmt, string::String, vec, vec::Vec};

#[cfg(feature = "alloc")]
use libc_print::std_name::{dbg, eprintln, println};

#[no_mangle]
#[cfg(feature = "alloc")]
pub extern "C" fn test_vec() {
    let mut v = vec![1, 2, 3];
    let res = v.pop();
}

#[no_mangle]
#[cfg(feature = "alloc")]
pub extern "C" fn format() {
    println!("Hi");
}

/// # Safety
/// Very safe
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
pub extern "C" fn rand_u64() -> u64 {
    use nanorand::{Rng, WyRand};
    let mut rng = WyRand::new();
    rng.generate::<u64>()
}

#[no_mangle]
/// # Safety
/// Safe
pub unsafe extern "C" fn shuffle(
    array_pointer: *mut CULongLong,
    size: usize,
    c_array: &mut CArray,
) {
    use nanorand::{Rng, WyRand};
    let mut rng = WyRand::new();
    let mut items = unsafe { Vec::from_raw_parts(array_pointer, size, size) };
    rng.shuffle(&mut items);
    let (ptr, size, _capacity) = items.into_raw_parts();
    c_array.ptr = ptr;
    c_array.size = size;
}

#[repr(C)]
pub struct CArray {
    ptr: *mut CULongLong,
    size: usize,
}

#[repr(C)]
pub struct CVec {
    ptr: *mut CULongLong,
    size: usize,
    capacity: usize,
}

#[no_mangle]
pub extern "C" fn print(print_str: *const CChar) {
    let str_to_print = cchar_to_cstr(print_str);
    println!("{}", str_to_print);
}

pub mod syscall;
pub use syscall::*;

fn cchar_to_cstr(c: *const CChar) -> &'static str {
    unsafe { core::ffi::CStr::from_ptr(c) }
        .to_str()
        .expect("Couldn't convert from CChar to CStr")
}

#[no_mangle]
#[cfg(feature = "alloc")]
pub extern "C" fn print_async(print_str: *const CChar) {
    use nostd_async::{Runtime, Task};

    let runtime = Runtime::new();

    let mut task = Task::new(async { println!("{}", cchar_to_cstr(print_str)) });

    let handle = task.spawn(&runtime);

    handle.join();
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
        let input = CStr::from_bytes_with_nul(b"\0").unwrap();
        assert_eq!(unsafe { strlen(input.as_ptr()) }, 0);
    }

    #[test]
    fn syscalls() {
        let string = "Hello World\n";

        let ptr = string.as_ptr() as usize;
        let len = string.len();
        write(1usize, ptr, len);
    }
}
