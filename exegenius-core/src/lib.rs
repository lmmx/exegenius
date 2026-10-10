//! Core library for exegenius.
//!
//! The pipeline turns source documents into a wiki of linked explanatory pages:
//!
//! 1. [`extract`]: a model decomposes the sources into a concept graph; code validates it.
//! 2. [`plan`]: code computes the pages needed to reach the target, in reading order.
//! 3. [`write`]: a model writes each page body; code checks its links.
//! 4. [`audit`]: a model lists what the reader cannot follow on each page; code turns each
//!    gap into a graph edit or a rewrite note.
//! 5. [`build`]: code writes the docs tree and `zensical.toml`.
//!
//! Every model call goes through the [`model::Model`] trait, so the whole pipeline runs
//! offline against recorded responses in tests.

pub mod audit;
pub mod build;
pub mod config;
pub mod error;
pub mod extract;
pub mod graph;
pub mod model;
pub mod plan;
pub mod project;
pub mod text;
pub mod write;

pub use error::{Error, Result};

/// Progress messages from long-running steps.
pub type Log<'a> = &'a mut dyn FnMut(&str);
