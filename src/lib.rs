#[cfg(feature = "serde")]
pub mod serde;

/// Serializable Owned/Borrowed.
///
/// A type representing borrowed or owned data, similar to `Cow`, but without the requirement
/// that the inner type must implement the `Clone` trait, making it easily serializable and deserializable.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Sob<'a, T> {
    /// Borrowed data.
    Borrowed(&'a T),
    /// Owned data.
    Owned(T),
}
