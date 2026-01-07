//! Slab Allocator Implementation
//!
//! A no_std compatible slab allocator for the ESGI 4A exam.
//!
//! # Author
//! Noella IKIREZI - ESGI 4A

// Pour la soumission finale (no_std), decommenter:
// #![no_std]
// #![feature(alloc_error_handler)]

#![allow(dead_code)]

extern crate alloc;

pub mod slab;

pub use slab::SlabAllocator;
