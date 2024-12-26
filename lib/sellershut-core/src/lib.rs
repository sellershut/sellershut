#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "users")]
#[cfg_attr(docsrs, doc(cfg(feature = "users")))]
pub mod users;

#[cfg(any(feature = "users", feature = "categories"))]
pub mod google;

#[cfg(feature = "categories")]
#[cfg_attr(docsrs, doc(cfg(feature = "categories")))]
pub mod categories;

#[cfg(feature = "categories")]
#[cfg_attr(docsrs, doc(cfg(feature = "categories")))]
/// Resuable utilities
pub mod common;
