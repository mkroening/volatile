#![no_std]
#![cfg_attr(feature = "unstable", feature(core_intrinsics))]
#![cfg_attr(feature = "unstable", feature(slice_range))]
#![cfg_attr(feature = "unstable", feature(slice_ptr_get))]
#![cfg_attr(feature = "unstable", feature(slice_ptr_len))]
#![cfg_attr(feature = "very_unstable", feature(const_slice_ptr_len))]
#![cfg_attr(feature = "very_unstable", feature(const_trait_impl))]
#![cfg_attr(feature = "very_unstable", feature(const_mut_refs))]
#![cfg_attr(feature = "very_unstable", feature(inline_const))]
#![cfg_attr(feature = "very_unstable", feature(unboxed_closures))]
#![cfg_attr(feature = "very_unstable", feature(fn_traits))]
#![cfg_attr(all(feature = "unstable", test), feature(slice_as_chunks))]
#![warn(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

pub use volatile_ptr::VolatilePtr;
pub use volatile_ref::VolatileRef;

pub mod access;
mod macros;
mod volatile_ptr;
mod volatile_ref;
