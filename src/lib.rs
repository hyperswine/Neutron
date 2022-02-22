#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), feature(alloc_error_handler))]
// SUPPRESS WARNINGS
#![allow(dead_code)]

// NON ARCH DEPENDENT CODE

#[cfg(not(test))]
extern crate alloc;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod drivers;
pub mod filesystem;
pub mod kext;
pub mod process;
pub mod services;
pub mod types;

// ARCH DEPENDENT CODE

// Kernel Manager and ARCH Specific
pub mod kernel;
