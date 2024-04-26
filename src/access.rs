//! Marker types for limiting access.

/// Sealed trait that is implemented for the types in this module.
pub trait Access: Copy + Default + private::Sealed {
    /// Reduced access level to safely share the corresponding value.
    type RestrictShared: Access;
}

/// Helper trait that is implemented by [`ReadWrite`] and [`ReadOnly`].
pub trait Readable: Copy + Default + private::Sealed {
    /// Reduced access level to safely share the corresponding value.
    type RestrictShared: Readable + Access;
}

/// Helper trait that is implemented by [`ReadWrite`] and [`WriteOnly`].
pub trait Writable: Access + private::Sealed {}

/// Implemented for access types that permit copying of `VolatileRef`.
pub trait Copyable: private::Sealed {}

impl<T> Access for T
where
    T: Readable + Default + Copy,
{
    type RestrictShared = <T as Readable>::RestrictShared;
}

/// Zero-sized marker type for allowing both read and write access.
#[derive(Debug, Default, Copy, Clone)]
pub struct ReadWrite;
impl Readable for ReadWrite {
    type RestrictShared = ReadOnly;
}
impl Writable for ReadWrite {}

/// Zero-sized marker type for allowing only read access.
#[derive(Debug, Default, Copy, Clone)]
pub struct ReadOnly;
impl Readable for ReadOnly {
    type RestrictShared = ReadOnly;
}
impl Copyable for ReadOnly {}

/// Zero-sized marker type for allowing only write access.
#[derive(Debug, Default, Copy, Clone)]
pub struct WriteOnly;
impl Access for WriteOnly {
    type RestrictShared = NoAccess;
}
impl Writable for WriteOnly {}

/// Zero-sized marker type that grants no access.
#[derive(Debug, Default, Copy, Clone)]
pub struct NoAccess;
impl Access for NoAccess {
    type RestrictShared = NoAccess;
}
impl Copyable for NoAccess {}

mod private {
    pub trait Sealed {}

    impl Sealed for super::ReadWrite {}
    impl Sealed for super::ReadOnly {}
    impl Sealed for super::WriteOnly {}
    impl Sealed for super::NoAccess {}
}
