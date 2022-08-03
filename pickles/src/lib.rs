/// Expression framework
mod expr;

/// Generic kimchi types/methods
pub mod kimchi;

/// Accumulation for IPA
mod ipa;

/// Variable types
mod types;

/// Represents a union of two constraint systems
/// and the consistency between public inputs "exported/passed" between the two sides
pub mod context;

/// Fiat-Shamir abstraction
mod transcript;

mod commitment;

mod util;

/// Combines multiple instances of the Kimchi verifier
/// and enforces the correct deferring of the variables.
mod glue;