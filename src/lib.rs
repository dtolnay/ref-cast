//! # Basic example
//!
//! ```rust
//! #[macro_use]
//! extern crate ref_cast;
//! use ref_cast::RefCast;
//!
//! #[derive(RefCast)]
//! #[repr(C)]
//! struct U(String);
//!
//! fn main() {
//!     let s = String::new();
//!
//!     // Safely cast from `&String` to `&U`.
//!     let u = U::ref_cast(&s);
//! }
//! ```
//!
//! # Realistic example
//!
//! Suppose we have a multidimensional array represented in a flat buffer in
//! row-major order for performance reasons, but we want to expose an indexing
//! operation that works in column-major order because it is more intuitive in
//! the context of our application.
//!
//! ```rust
//! const MAP_WIDTH: usize = 4;
//!
//! struct Tile(u8);
//!
//! struct TileMap {
//!     storage: Vec<Tile>,
//! }
//!
//! // `tilemap[x][y]` should give us `tilemap.storage[y * MAP_WIDTH + x]`.
//! ```
//!
//! The signature of the [`Index`] trait in Rust is such that the output is
//! forced to be borrowed from the type being indexed. So something like the
//! following is not going to work.
//!
//! [`Index`]: https://doc.rust-lang.org/std/ops/trait.Index.html
//!
//! ```rust
//! # const MAP_WIDTH: usize = 4;
//! #
//! # struct Tile(u8);
//! #
//! # struct TileMap {
//! #     storage: Vec<Tile>,
//! # }
//! #
//! struct Column<'a> {
//!     tilemap: &'a TileMap,
//!     x: usize,
//! }
//!
//! # mod index1 {
//! #     use super::{TileMap, Column, MAP_WIDTH};
//! #
//! #     trait Index<Idx> {
//! #         fn index(&self, idx: Idx) -> Column;
//! #     }
//! #
//! // Does not work! The output of Index must be a reference that is
//! // borrowed from self. Here the type Column is not a reference.
//! impl Index<usize> for TileMap {
//!     fn index(&self, x: usize) -> Column {
//!         assert!(x < MAP_WIDTH);
//!         Column { tilemap: self, x }
//!     }
//! }
//! # }
//!
//! # mod index2 {
//! #     use super::{Column, Tile, MAP_WIDTH};
//! #     use std::ops::Index;
//! #
//! impl<'a> Index<usize> for Column<'a> {
//!     # type Output = Tile;
//!     fn index(&self, y: usize) -> &Tile {
//!         &self.tilemap.storage[y * MAP_WIDTH + self.x]
//!     }
//! }
//! # }
//! #
//! # fn main() {}
//! ```
//!
//! Here is a working approach using `RefCast`.
//!
//! ```rust
//! # #[macro_use]
//! # extern crate ref_cast;
//! # use ref_cast::RefCast;
//! #
//! # use std::ops::Index;
//! #
//! # const MAP_WIDTH: usize = 4;
//! #
//! # struct Tile(u8);
//! #
//! # struct TileMap {
//! #     storage: Vec<Tile>,
//! # }
//! #
//! #[derive(RefCast)]
//! #[repr(C)]
//! struct Strided([Tile]);
//!
//! // Implement `tilemap[x][y]` as `tilemap[x..][y * MAP_WIDTH]`.
//! impl Index<usize> for TileMap {
//!     type Output = Strided;
//!     fn index(&self, x: usize) -> &Self::Output {
//!         assert!(x < MAP_WIDTH);
//!         RefCast::ref_cast(&self.storage[x..])
//!     }
//! }
//!
//! impl Index<usize> for Strided {
//!     type Output = Tile;
//!     fn index(&self, y: usize) -> &Self::Output {
//!         &self.0[y * MAP_WIDTH]
//!     }
//! }
//! #
//! # fn main() {}
//! ```

#![doc(html_root_url = "https://docs.rs/ref-cast/0.2.4")]

#[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
#[allow(unused_imports)]
#[macro_use]
extern crate ref_cast_impl;
#[doc(hidden)]
pub use ref_cast_impl::*;

/// Safely cast `&T` to `&U` where the struct `U` contains a single field of
/// type `T`.
///
/// ```rust
/// # #[macro_use]
/// # extern crate ref_cast;
/// #
/// // `&String` can be cast to `&U`.
/// #[derive(RefCast)]
/// #[repr(C)]
/// struct U(String);
///
/// // `&T` can be cast to `&V<T>`.
/// #[derive(RefCast)]
/// #[repr(C)]
/// struct V<T> {
///     t: T,
/// }
/// #
/// # fn main() {}
/// ```
///
/// See the crate-level documentation for usage examples!
pub trait RefCast {
    type From: ?Sized;
    fn ref_cast(from: &Self::From) -> &Self;
    fn ref_cast_mut(from: &mut Self::From) -> &mut Self;
}
