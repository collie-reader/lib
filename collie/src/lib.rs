#![warn(clippy::all)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Collie
//!
//! Collie is both a feed reader application in its own right,
//! and a set of specifications for implementing a minimal feed reader application.
//!
//! ## Usage
//!
//! - To implement backend system to a feed reader application.
//! - To write a server application for serving feed data.
//!
//! ## Features
//!
//! - `core` includes core features. (enabled by default)
//! - `auth` enables authentication features.

pub use collie_core::*;

#[cfg(feature = "auth")]
#[cfg_attr(docsrs, doc(cfg(feature = "auth")))]
pub mod auth {
    pub use collie_auth::*;
}
