#[cfg(all(
    any(target_os = "linux", target_os = "android"),
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
use core::arch::asm;

#[cfg(not(all(
    any(target_os = "linux", target_os = "android"),
    any(target_arch = "x86_64", target_arch = "aarch64")
)))]
compile_error!("Only works on linux x86_64 or linux aarch64");

macro_rules! syscall0 {
    ($name:ident, $nr:expr) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
                compile_error!("not implemented");
            }
        }
    };
}

macro_rules! syscall1 {
    ($name:ident, $nr:expr) => {
        #[no_mangle]
        pub extern "C" fn $name(arg1: usize) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr,
                    in("x0") arg1,
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1,
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
                compile_error!("not implemented");
            }
        }
    }
}

macro_rules! syscall2 {
    ($name:ident, $nr:expr) => {
        #[no_mangle]
        pub extern "C" fn $name(arg1: usize, arg2: usize) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr,
                    in("x0") arg1,
                    in("x1") arg2,
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1,
                    in("rsi") arg2,
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
                compile_error!("not implemented");
            }
        }
    }
}

macro_rules! syscall3 {
    ($name:ident, $nr:expr) => {
        #[no_mangle]
        pub extern "C" fn $name(arg1: usize, arg2: usize, arg3: usize) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr,
                    in("x0") arg1,
                    in("x1") arg2,
                    in("x2") arg3,
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1,
                    in("rsi") arg2,
                    in("rdx") arg3,
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
                compile_error!("not implemented");
            }
        }
    }
}

macro_rules! syscall4 {
    ($name:ident, $nr:expr) => {
        #[no_mangle]
        pub extern "C" fn $name(arg1: usize, arg2: usize, arg3: usize, arg4: usize) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr,
                    in("x0") arg1,
                    in("x1") arg2,
                    in("x2") arg3,
                    in("x3") arg4,
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1,
                    in("rsi") arg2,
                    in("rdx") arg3,
                    in("r10") arg4,
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
                compile_error!("not implemented");
            }
        }
    }
}

macro_rules! syscall5 {
    ($name:ident, $nr:expr) => {
        #[no_mangle]
        pub extern "C" fn $name(arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr,
                    in("x0") arg1,
                    in("x1") arg2,
                    in("x2") arg3,
                    in("x3") arg4,
                    in("x4") arg5,
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1,
                    in("rsi") arg2,
                    in("rdx") arg3,
                    in("r10") arg4,
                    in("r9") arg5,
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
                compile_error!("not implemented");
            }
        }
    }
}

macro_rules! syscall6 {
    ($name:ident, $nr:expr) => {
        #[no_mangle]
        pub extern "C" fn $name(arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr,
                    in("x0") arg1,
                    in("x1") arg2,
                    in("x2") arg3,
                    in("x3") arg4,
                    in("x4") arg5,
                    in("x5") arg6,
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1,
                    in("rsi") arg2,
                    in("rdx") arg3,
                    in("r10") arg4,
                    in("r9") arg5,
                    in("r8") arg6,
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
                compile_error!("not implemented");
            }
        }
    }
}

#[cfg(all(
    target_arch = "aarch64",
    any(target_os = "linux", target_os = "android")
))]
syscall0!(exit, 93);
#[cfg(all(
    target_arch = "aarch64",
    any(target_os = "linux", target_os = "android")
))]
syscall3!(write, 64);

#[cfg(all(
    target_arch = "x86_64",
    any(target_os = "linux", target_os = "android")
))]
syscall0!(exit, 60);
#[cfg(all(
    target_arch = "x86_64",
    any(target_os = "linux", target_os = "android")
))]
syscall3!(write, 1);
