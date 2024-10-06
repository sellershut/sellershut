#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "categories")]
pub mod categories;

#[cfg(feature = "users")]
pub mod users;

#[cfg(feature = "listings")]
pub mod listings;

#[cfg(all(
    feature = "base",
    any(feature = "users", feature = "listings", feature = "categories")
))]
pub mod google;

#[cfg(feature = "categories")]
#[cfg_attr(docsrs, doc(cfg(feature = "categories")))]
/// Resuable utilities
pub mod common;
