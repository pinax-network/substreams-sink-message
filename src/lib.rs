//! ### Example
//! ```
//! use std::collections::HashMap;
//! use substreams_sink_winston::{Logger, Meta, LoggerOperations};
//!
//! // Initialize Winston Logger operations container
//! let mut log_ops: LoggerOperations = Default::default();
//!
//! // Create Logger
//! // ==============
//! let mut logger = Logger::new("user-service");
//!
//! // Informational: informational messages
//! log_ops.push(logger.info("message"));
//!
//! // Error: error conditions
//! log_ops.push(logger.error("error message"));
//!
//! // Include Metadata
//! let meta = Meta::from(vec!(["key", "value"]));
//! log_ops.push(logger.info("message").with(meta));
//! ```
#[path = "pb/pinax.substreams.sink.winston.v1.rs"]
#[allow(dead_code)]
pub mod pb;
pub use self::pb::*;

mod helpers;
mod meta;
pub use self::meta::*;
mod logger;
pub use self::logger::*;
