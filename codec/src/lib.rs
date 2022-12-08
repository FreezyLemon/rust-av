//! Contains utilities for encoding and decoding video and audio formats.

#![deny(missing_docs, clippy::undocumented_unsafe_blocks)]

#![feature(error_in_core)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

/// Data structs shared between encoders and decoders.
pub mod common;
/// Utilities for decoding video and audio formats.
pub mod decoder;
/// Utilities for encoding video and audio formats.
pub mod encoder;
/// Error types.
pub mod error;
