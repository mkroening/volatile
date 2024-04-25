//! Provides volatile wrapper types for raw pointers.
//!
//! The volatile wrapper types in this crate wrap a pointer to any [`Copy`]-able
//! type and provide volatile memory access to wrapped value. Volatile memory accesses are
//! never optimized away by the compiler, and are useful in many low-level systems programming
//! and concurrent contexts.
//!
//! This crate provides two different wrapper types: [`VolatilePtr`] and [`VolatileRef`]. The
//! difference between the two types is that the former behaves like a raw pointer, while the
//! latter behaves like a Rust reference type. For example, `VolatilePtr` can be freely copied,
//! but not sent across threads because this could introduce mutable aliasing. The `VolatileRef`
//! type, on the other hand, requires exclusive access for mutation, so that sharing it across
//! thread boundaries is safe.
//!
//! Both wrapper types *do not* enforce any atomicity guarantees; to also get atomicity, consider
//! looking at the `Atomic` wrapper types found in `libcore` or `libstd`.
//!
//! ## Why is there no `VolatileCell`?
//!
//! Many people expressed interest in a `VolatileCell` type, i.e. a transparent wrapper type that
//! owns the wrapped value. Such a type would be similar to [`core::cell::Cell`], with the
//! difference that all methods are volatile.
//!
//! Unfortunately, it is not sound to implement such a `VolatileCell` type in Rust. The reason
//! is that Rust and LLVM consider `&` and `&mut` references as _dereferencable_. This means that
//! the compiler is allowed to freely access the referenced value without any restrictions. So
//! no matter how a `VolatileCell` type is implemented, the compiler is allowed to perform
//! non-volatile read operations of the contained value, which can lead to unexpected (or even
//! undefined?) behavior. For more details, see the discussion
//! [in our repository](https://github.com/rust-osdev/volatile/issues/31)
//! and
//! [in the `unsafe-code-guidelines` repository](https://github.com/rust-lang/unsafe-code-guidelines/issues/411).

#![no_std]
#![cfg_attr(feature = "unstable", feature(core_intrinsics))]
#![cfg_attr(feature = "unstable", feature(slice_range))]
#![cfg_attr(feature = "unstable", feature(slice_ptr_get))]
#![cfg_attr(feature = "very_unstable", feature(const_trait_impl))]
#![cfg_attr(feature = "very_unstable", feature(unboxed_closures))]
#![cfg_attr(feature = "very_unstable", feature(fn_traits))]
#![cfg_attr(feature = "very_unstable", feature(effects))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = "unstable", allow(internal_features))]
#![warn(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

/// A derive macro for method-based accesses to volatile structures.
///
/// This macro allows you to access the fields of a volatile structure via methods that enforce access limitations.
/// It is also more easily chainable than [`map_field`].
///
/// <div class="warning">
///
/// This macro generates and implements a new `{T}VolatileFieldAccess` trait, that you have to import if used from other modules.
/// Currently, the trait is only implemented for `VolatilePtr<'_, _, ReadWrite>`.
///
/// </div>
///
/// # Examples
///
/// ```
/// use volatile::access::ReadOnly;
/// use volatile::{VolatileFieldAccess, VolatilePtr, VolatileRef};
///
/// #[repr(C)]
/// #[derive(VolatileFieldAccess, Default)]
/// pub struct DeviceConfig {
///     feature_select: u32,
///     #[access(ReadOnly)]
///     feature: u32,
/// }
///
/// let mut device_config = DeviceConfig::default();
/// let mut volatile_ref = VolatileRef::from_mut_ref(&mut device_config);
/// let mut volatile_ptr = volatile_ref.as_mut_ptr();
///
/// volatile_ptr.feature_select().write(42);
/// assert_eq!(volatile_ptr.feature_select().read(), 42);
///
/// // This does not compile, because we specified `#[access(ReadOnly)]` for this field.
/// // volatile_ptr.feature().write(42);
///
/// // A real device might have changed the value, though.
/// assert_eq!(volatile_ptr.feature().read(), 0);
/// ```
///
/// # Details
///
/// This macro generates a new trait (`{T}VolatileFieldAccess`) and implements it for `VolatilePtr<'a, T, ReadWrite>`.
/// The example above results in (roughly) the following code:
///
/// ```
/// pub trait DeviceConfigVolatileFieldAccess<'a> {
///     fn feature_select(self) -> VolatilePtr<'a, u32, ReadWrite>;
///
///     fn feature(self) -> VolatilePtr<'a, u32, ReadOnly>;
/// }
///
/// impl<'a> DeviceConfigVolatileFieldAccess<'a> for VolatilePtr<'a, DeviceConfig, ReadWrite> {
///     fn feature_select(self) -> VolatilePtr<'a, u32, ReadWrite> {
///         map_field!(self.feature_select).restrict()
///     }
///
///     fn feature(self) -> VolatilePtr<'a, u32, ReadOnly> {
///         map_field!(self.feature).restrict()
///     }
/// }
/// ```
#[cfg(feature = "derive")]
pub use volatile_macro::VolatileFieldAccess;

pub use volatile_ptr::VolatilePtr;
pub use volatile_ref::VolatileRef;

pub mod access;
mod volatile_ptr;
mod volatile_ref;
