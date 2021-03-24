//! Specify the format and how lenient the deserialization is

/// Specify how to serialize/deserialize a type
///
/// The format specifier allows to configure how a value is serialized/deserialized.
/// For example, you can serialize a timestamp as an integer using the UNIX epoch, as a string containing an integer, or as a string using ISO 8601.
/// This [`Format`] traits allows more flexibility in configuring the format without the need to create a new type for each case.
pub trait Format {}

macro_rules! impl_format {
    ($(#[$attr:meta] $t:ty)*) => {
        $(
            #[$attr]
            impl Format for $t {}
        )*
    };
}
macro_rules! create_format {
    ($(#[$attr:meta] $t:ident)*) => {
        $(
            #[$attr]
            #[derive(Copy, Clone, Debug, Default)]
            pub struct $t;
            impl_format!(#[$attr] $t);
        )*
    };
}
impl_format!(
    /// Serialize into an i8
    i8
    /// Serialize into a u8
    u8
    /// Serialize into an i16
    i16
    /// Serialize into a u16
    u16
    /// Serialize into an i32
    i32
    /// Serialize into a u32
    u32
    /// Serialize into an i64
    i64
    /// Serialize into a u64
    u64

    /// Serialize into a f32
    f32
    /// Serialize into a f64
    f64

    /// Serialize into a bool
    bool

    /// Serialize into a String
    String
);
serde::serde_if_integer128!(impl_format!(
    /// Serialize into an i128
    i128
    /// Serialize into a u128
    u128
););

create_format!(
    /// Use uppercase characters
    Uppercase
    /// Use lowercase characters
    Lowercase

    /// Use in combination with [`OneOrMany`]. Emit single element for lists of size 1.
    PreferOne
    /// Use in combination with [`OneOrMany`]. Always emit the list form.
    PreferMany
);

/// Specify how lenient the deserialization process should be
///
/// Formats which make use of this trait should specify how it affects the deserialization behavior.
pub trait Strictness {}

/// Use strict deserialization behavior, see [`Strictness`].
#[derive(Copy, Clone, Debug, Default)]
pub struct Strict;
impl Strictness for Strict {}

/// Use a flexible deserialization behavior, see [`Strictness`].
#[derive(Copy, Clone, Debug, Default)]
pub struct Flexible;
impl Strictness for Flexible {}
