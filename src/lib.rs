#![warn(clippy::all)]
mod pages;
pub use pages::*;

mod wrap_pages;
pub use wrap_pages::WrapPages;

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::*;