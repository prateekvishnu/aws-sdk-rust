#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>AWS CloudHSM Service</fullname>
//! <p>This is documentation for <b>AWS CloudHSM Classic</b>. For
//! more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM
//! Classic FAQs</a>, the <a href="https://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS
//! CloudHSM Classic User Guide</a>, and the <a href="https://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p>
//! <p>
//! <b>For information about the current version of AWS
//! CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the
//! <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>,
//! and the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API
//! Reference</a>.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("cloudhsm", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
