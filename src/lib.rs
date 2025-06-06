#![no_std]
#![doc = include_str!("../README.md")]
#![allow(non_snake_case, non_camel_case_types)]

// Allow usage of `alloc` crate for heap-allocated types.
extern crate alloc;

// Internal modules
mod address;
mod wrappers;
mod macros;
mod module;
mod syscall;
mod utils;
mod str;

/// Structures and types used across the library.
pub mod data;

/// Runtime hash functions.
pub mod hash;

/// PE Parsing
pub mod parse;

/// Hardware breakpoint management utilities (only for x86/x86_64 targets).
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod breakpoint;

/// Custom panic handler support (requires `dinvk_panic` feature).
#[cfg(feature = "dinvk_panic")]
pub mod panic;

/// Heap allocator using Windows native APIs (requires `alloc` feature).
#[cfg(feature = "alloc")]
pub mod allocator;

// Re-exports for easier usage
pub use syscall::*;
pub use wrappers::*;
pub use address::*;
pub use module::*;
pub use utils::*;
pub use str::*;