//! Formatter for logging `tracing_subscriber` events in logfmt format.
//!
//! Use as a formatting layer in tracing subscriber:
//! ```rust
//! use tracing::dispatcher::{self, Dispatch};
//! use tracing_subscriber::Registry;
//! use tracing_subscriber::layer::SubscriberExt;
//!
//! let subscriber = Registry::default()
//!     .with(tracing_logfmt::layer());
//!
//! dispatcher::set_global_default(Dispatch::new(subscriber))
//!     .expect("Global logger has already been set!");
//! ```
//!
//! ## `log` records
//!
//! The `tracing-log` feature, which is enabled by default, `tracing-logfmt`
//! can support logs coming from the `log` crate.
//!
//! Unlike `tracing-subscriber`, you must set your own
//! [`LogTracer`](tracing_log::LogTracer). Do it right after setting the
//! dispatcher:
//!
//! ```rust
//! # use tracing::dispatcher::{self, Dispatch};
//! # use tracing::level_filters::LevelFilter;
//! # use tracing_log::{AsLog, LogTracer};
//! # use tracing_subscriber::Registry;
//! # use tracing_subscriber::layer::SubscriberExt;
//!
//! let subscriber = Registry::default()
//!     .with(tracing_logfmt::layer());
//!
//! dispatcher::set_global_default(Dispatch::new(subscriber))
//!     .expect("Global logger has already been set!");
//!
//! LogTracer::builder()
//!     .with_max_level(LevelFilter::current().as_log())
//!     .init()
//!     .expect("a `log` logger has already been set");
//! ```
//!
//! [`tracing-log`]: https://docs.rs/tracing-log

#![deny(unreachable_pub)]

mod formatter;
mod serializer;

pub use crate::formatter::builder::{builder, Builder};
pub use crate::formatter::{layer, EventsFormatter, FieldsFormatter};
pub use tracing_subscriber::fmt::format::FmtSpan;
