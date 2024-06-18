#![allow(missing_docs)] // FIXME: Document this

pub use self::hbs_renderer::HtmlHandlebars;

pub mod hbs_renderer;
mod helpers;

#[cfg(feature = "search")]
pub mod search;
