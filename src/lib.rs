//! SolidWorks document interoperability, starting with the 2015+ container.
//!
//! ```no_run
//! let bytes = std::fs::read("part.sldprt")?;
//! let document = bessemer::Document::from_bytes(&bytes)?;
//! let preview = document.read_stream("PreviewPNG")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod archive;
mod archive_codec;
pub mod biography;
pub mod block_stream;
pub mod composer;
pub mod compound;
mod compression;
pub mod display_lists;
mod document;
mod error;
pub mod header;
pub mod history;
pub mod inspection;
pub mod metadata;
pub mod model_archives;
pub mod parasolid;
pub mod preview;
mod records;
pub mod tessellation;
mod writer;

pub use document::{Document, StreamInfo};
pub use error::{BessemerError, FailureClass, Result};
