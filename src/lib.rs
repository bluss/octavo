//! Fancy pure Rust crypto and hash library.
//!
//! The main reason this exists is so I can learn about cryptography.
//!
//! If you're asking "We have rust-crypto, why the hell do we need a new
//! library?"), `rust-crypto` was (at the time of Octavio's creation) big pile of… mess. To be honest
//! I doesn't hate `rust-crypto` but I wanted something that will be organized in fancy way, not
//! another OpenSSL (but written in Rust, so probably less buggy).
//!
//! Anther reason is that I want Octavo to became highly configurable via Cargo features. Each
//! cipher, hash family, MAC, etc. should be behind a feature.
//!
//! Oh… one more thing. I want to create C wrapper for Octavo (maybe even OpenSSL compatible), so
//! that you could use this beautiful peace of crap.
//!
//! Happy coding!
//!
//! ## Examples
//!
//!

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(not(feature = "lints"), allow(unknown_lints))]

#![deny(trivial_casts, unreachable_code, while_true)]
#![warn(unused_qualifications)]

extern crate byteorder;

#[macro_use] extern crate log;

#[cfg(feature = "num")] extern crate num;
#[cfg(feature = "rand")] extern crate rand;

pub mod crypto;
pub mod digest;
pub mod mac;

mod utils;
