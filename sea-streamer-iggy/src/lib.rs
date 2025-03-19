//! ### `sea-streamer-iggy`: Iggy  Backend
//!
//! This is the Iggy backend implementation for SeaStreamer.
//! This crate provides a comprehensive type system that makes working with Iggy easier and safer.
//!
//! First of all, all API (many are sync) are properly wrapped as async. Methods are also marked `&mut` to eliminate possible race conditions.
//!
//! `IggyConsumerOptions` has typed parameters.
//!
//! `IggyConsumer` allows you to `seek` to point in time, `rewind` to particular offset, and `commit` message read.
//!
//! `IggyProducer` allows you to `await` a send `Receipt` or discard it if you are uninterested. You can also flush the Producer.
//!
//! `IggyStreamer` allows you to flush all producers on `disconnect`.
//!
//! See [tests](https://github.com/SeaQL/sea-streamer/blob/main/sea-streamer-iggy/tests/consumer.rs) for an illustration of the stream semantics.
//!
//!This crate depends on [`iggy`](https://docs.iggy.rs/),
//!
//! SDK Reference: <https://docs.iggy.rs/introduction/high-level-sdk>

#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/SeaQL/sea-streamer/main/docs/SeaQL icon.png"
)]

/// The default Iggy port number
pub const IGGY_PORT: u16 = 8090;

/// The default timeout, if needed but unspecified
pub const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);

#[cfg(all(feature = "runtime-async-std", feature = "runtime-tokio"))]
compile_error!("'runtime-async-std' and 'runtime-tokio' cannot be enabled at the same time");

mod cluster;
mod consumer;
mod error;
mod host;
mod producer;
mod runtime;
mod streamer;

use cluster::*;
pub use consumer::*;
pub use error::*;
pub use host::*;
pub use producer::*;
pub use runtime::*;
pub use streamer::*;

/// Re-export types from `iggy`
pub mod export {
    pub use iggy;
}

macro_rules! impl_into_string {
    ($name:ident) => {
        impl From<$name> for String {
            fn from(o: $name) -> Self {
                o.as_str().to_owned()
            }
        }
    };
}

pub(crate) use impl_into_string;
