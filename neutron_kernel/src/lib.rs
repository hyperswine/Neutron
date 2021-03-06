#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![allow(named_asm_labels)]
#![feature(asm_const)]
#![feature(step_trait)]
#![feature(trait_alias)]
#![feature(core_intrinsics)]
#![feature(derive_default_enum)]

// -----------------------
// CRATE WIDE API
// -----------------------

// NOTE: for tests, just use extern crate alloc and link to the hosts' alloc
extern crate alloc;
extern crate goblin;
extern crate log;

// -----------------------
// NON ARCH DEPENDENT CODE
// -----------------------

pub mod drivers;
pub mod exception;
pub mod filesystem;
pub mod kmod;
pub mod process;
pub mod services;
pub mod types;
// ALLOCATOR AND THE REST
pub mod memory;
// Pass the torch
pub mod userland;
pub mod kernel;

// -----------------------
// ARCH DEPENDENT CODE
// -----------------------

pub mod arch;
