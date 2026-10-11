//! Core library for exegenius.
//!
//! exegenius turns source documents into a wiki of linked explanatory pages for a stated
//! reader. It makes no model calls itself: it holds the state (concept graph, plan, pages,
//! audit results) and hands out tasks, and an agent such as Claude Code does the reading
//! and writing those tasks ask for. Every answer is checked by code before it is applied.
//!
//! The task kinds, in the order they are handed out:
//!
//! 1. [`extract`]: decompose the sources into a concept graph (and later resolve the
//!    candidate concepts audits find).
//! 2. [`write`]: write each page body, in the reading order from [`plan`].
//! 3. [`audit`]: list what the reader cannot follow on each page, given exactly what the
//!    reader knows at that point.
//! 4. [`outline`] and [`notation`]: once the pages are final, group them into parts (folding
//!    minor pages into a glossary) and write the front page, and list the notation.
//!
//! [`task`] decides what to hand out next and applies submitted answers; [`build`] writes
//! the docs tree and `zensical.toml` once no task is left.

pub mod audit;
pub mod autolink;
pub mod build;
pub mod cite;
pub mod config;
pub mod error;
pub mod extract;
pub mod grade;
pub mod graph;
pub mod notation;
pub mod outline;
pub mod plan;
pub mod project;
pub mod report;
pub mod simulate;
pub mod task;
pub mod text;
pub mod write;

pub use error::{Error, Result};
