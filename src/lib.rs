#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(vec_into_raw_parts)]
#![feature(ptr_sub_ptr)]
#![feature(c_variadic)]
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

// Ref CString
pub type CVoid = ::core::ffi::c_void;

#[cfg(feature = "alloc")]
use alloc::{fmt, slice, string::String, vec, vec::Vec};

use libc_print::std_name::{dbg, eprintln, print, println};

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
pub extern "C" fn printrs(s: *const CChar) {
    print!("{}", cchar_to_cstr(s));
}

#[no_mangle]
pub extern "C" fn printrsln(s: *const CChar) {
    println!("{}", cchar_to_cstr(s));
}

#[no_mangle]
/// # Safety
/// Safe
pub unsafe extern "C" fn rand_range(start: CULongLong, end: CULongLong) -> CULongLong {
    use nanorand::{Rng, WyRand};

    let mut rng = WyRand::new();
    rng.generate_range(start..=end)
}

#[no_mangle]
/// # Safety
/// Safe
pub unsafe extern "C" fn shuffle(
    array_pointer: *mut CULongLong,
    size: usize,
    c_array: &mut CArray,
) {
    use core::ops::Range;
    use nanorand::{Rng, WyRand};

    let mut rng = WyRand::new();
    let mut items = unsafe { slice::from_raw_parts_mut(array_pointer, size) };

    rng.shuffle(&mut items);

    let Range { start, end } = items.as_mut_ptr_range();

    c_array.ptr = start;
    c_array.size = end.sub_ptr(start) as u64;
}

#[repr(C)]
pub struct CArray {
    ptr: *mut CULongLong,
    size: CULongLong,
}

impl CArray {
    #[no_mangle]
    pub extern "C" fn new_carray() -> CArray {
        CArray {
            ptr: core::ptr::null_mut(),
            size: 0,
        }
    }
}

#[repr(C)]
pub struct CVec {
    ptr: *mut CULongLong,
    size: CULongLong,
    capacity: CULongLong,
}

impl CVec {
    #[no_mangle]
    pub extern "C" fn cvec_new() -> Self {
        Self::default()
    }

    #[no_mangle]
    pub extern "C" fn cvec_from(
        ptr: *mut CULongLong,
        size: CULongLong,
        capacity: CULongLong,
    ) -> Self {
        Self {
            ptr,
            size,
            capacity,
        }
    }

    #[no_mangle]
    pub extern "C" fn cvec_push(vector: &mut CVec, item: u64) {
        let mut v = unsafe {
            Vec::from_raw_parts(vector.ptr, vector.size as usize, vector.capacity as usize)
        };
        v.push(item);
        let (ptr, size, capacity) = v.into_raw_parts();
        vector.ptr = ptr;
        vector.size = size as u64;
        vector.capacity = capacity as u64;
    }

    #[no_mangle]
    pub extern "C" fn cvec_get(vector: &CVec, index: usize) -> u64 {
        unsafe { vector.ptr.offset(index.try_into().unwrap()).read() }
    }

    #[no_mangle]
    pub extern "C" fn cvec_set(vector: &CVec, index: usize, item: u64) {
        unsafe { vector.ptr.offset(index.try_into().unwrap()).write(item) }
    }

    #[no_mangle]
    pub extern "C" fn cvec_pop(vector: &mut CVec) -> u64 {
        vector.size -= 1;
        unsafe { vector.ptr.offset(vector.size.try_into().unwrap()).read() }
    }

    #[no_mangle]
    pub extern "C" fn cvec_last(vector: &CVec) -> u64 {
        unsafe { vector.ptr.offset(vector.size.try_into().unwrap()).read() }
    }

    #[no_mangle]
    pub extern "C" fn cvec_clear(vector: &mut CVec) {
        vector.size = 0;
        vector.capacity = 0;
        vector.ptr = core::ptr::null_mut();
    }
}

impl Default for CVec {
    fn default() -> Self {
        CVec {
            ptr: core::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }
}

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

    let mut task = Task::new(async {
        printrsln(print_str);
    });

    let handle = task.spawn(&runtime);

    handle.join();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strlen() {
        let input = CStr::from_bytes_with_nul(b"\0").unwrap();
        assert_eq!(unsafe { strlen(input.as_ptr()) }, 0);
    }
}
