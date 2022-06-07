#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>
//! <b>Introduction</b>
//! </p>
//! <p>The Amazon IVS Chat control-plane API enables you to create and manage Amazon IVS Chat
//! resources. You also need to integrate with the <a href="https://docs.aws.amazon.com/ivs/latest/chatmsgapireference/chat-messaging-api.html"> Amazon IVS Chat Messaging
//! API</a>, to enable users to interact with chat rooms in real time.</p>
//! <p>The API is an AWS regional service. For a list of supported regions and Amazon IVS Chat
//! HTTPS service endpoints, see the Amazon IVS Chat information on the <a href="https://docs.aws.amazon.com/general/latest/gr/ivs.html">Amazon IVS page</a> in the
//! <i>AWS General Reference</i>. </p>
//! <p>
//! <b>Notes on terminology:</b>
//! </p>
//! <ul>
//! <li>
//! <p>You create service applications using the Amazon IVS Chat API. We refer to these as
//! <i>applications</i>.</p>
//! </li>
//! <li>
//! <p>You create front-end client applications (browser and Android/iOS apps) using the
//! Amazon IVS Chat Messaging API. We refer to these as <i>clients</i>. </p>
//! </li>
//! </ul>
//!
//! <p>
//! <b>Resources</b>
//! </p>
//! <p>The following resource is part of Amazon IVS Chat:</p>
//! <ul>
//! <li>
//! <p>
//! <b>Room</b> — The central Amazon IVS Chat resource through
//! which clients connect to and exchange chat messages. See the Room endpoints for more
//! information.</p>
//! </li>
//! </ul>
//!
//! <p>
//! <b>API Access Security</b>
//! </p>
//! <p>Your Amazon IVS Chat applications (service applications and clients) must be authenticated
//! and authorized to access Amazon IVS Chat resources. Note the differences between these
//! concepts:</p>
//! <ul>
//! <li>
//! <p>
//! <i>Authentication</i> is about verifying identity. Requests to the
//! Amazon IVS Chat API must be signed to verify your identity.</p>
//! </li>
//! <li>
//! <p>
//! <i>Authorization</i> is about granting permissions. Your IAM roles need
//! to have permissions for Amazon IVS Chat API requests.</p>
//! </li>
//! </ul>
//! <p>Users (viewers) connect to a room using secure access tokens that you create using the
//! <a>CreateChatToken</a> endpoint through the AWS SDK. You call CreateChatToken for
//! every user’s chat session, passing identity and authorization information about the
//! user.</p>
//! <p>
//! <b>Signing API Requests</b>
//! </p>
//! <p>HTTP API requests must be signed with an AWS SigV4 signature using your AWS security
//! credentials. The AWS Command Line Interface (CLI) and the AWS SDKs take care of signing the
//! underlying API calls for you. However, if your application calls the Amazon IVS Chat HTTP API
//! directly, it’s your responsibility to sign the requests.</p>
//!
//!
//!
//!
//! <p>You generate a signature using valid AWS credentials for an IAM role that has permission
//! to perform the requested action. For example, DeleteMessage requests must be made using an IAM
//! role that has the <code>ivschat:DeleteMessage</code> permission.</p>
//! <p>For more information:</p>
//! <ul>
//! <li>
//! <p>Authentication and generating signatures — See <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html">Authenticating Requests
//! (Amazon Web Services Signature Version 4)</a> in the <i>Amazon Web Services
//! General Reference</i>.</p>
//! </li>
//! <li>
//! <p>Managing Amazon IVS permissions — See <a href="https://docs.aws.amazon.com/ivs/latest/userguide/security-iam.html">Identity and Access Management</a> on
//! the Security page of the <i>Amazon IVS User Guide</i>.</p>
//! </li>
//! </ul>
//!
//! <p>
//! <b>Messaging Endpoints</b>
//! </p>
//! <ul>
//! <li>
//! <p>
//! <a>DeleteMessage</a> — Sends an event to a specific room which
//! directs clients to delete a specific message; that is, unrender it from view and delete it
//! from the client’s chat history. This event’s <code>EventName</code> is
//! <code>aws:DELETE_MESSAGE</code>. This replicates the <a href="https://docs.aws.amazon.com/ivs/latest/chatmsgapireference/actions-deletemessage-publish.html"> DeleteMessage</a> WebSocket operation
//! in the Amazon IVS Chat Messaging API.</p>
//! </li>
//! <li>
//! <p>
//! <a>DisconnectUser</a> — Disconnects all connections using a specified
//! user ID from a room. This replicates the <a href="https://docs.aws.amazon.com/ivs/latest/chatmsgapireference/actions-disconnectuser-publish.html"> DisconnectUser</a> WebSocket operation
//! in the Amazon IVS Chat Messaging API.</p>
//! </li>
//! <li>
//! <p>
//! <a>SendEvent</a> — Sends an event to a room. Use this within your
//! application’s business logic to send events to clients of a room; e.g., to notify clients
//! to change the way the chat UI is rendered.</p>
//! </li>
//! </ul>
//! <p>
//! <b>Chat Token Endpoint</b>
//! </p>
//! <ul>
//! <li>
//! <p>
//! <a>CreateChatToken</a> — Creates an encrypted token that is used to
//! establish an individual WebSocket connection to a room. The token is valid for one minute,
//! and a connection (session) established with the token is valid for the specified
//! duration.</p>
//! </li>
//! </ul>
//!
//!
//! <p>
//! <b>Room Endpoints</b>
//! </p>
//! <ul>
//! <li>
//! <p>
//! <a>CreateRoom</a> — Creates a room that allows clients to connect and
//! pass messages.</p>
//! </li>
//! <li>
//! <p>
//! <a>DeleteRoom</a> — Deletes the specified room.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetRoom</a> — Gets the specified room.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListRooms</a> — Gets summary information about all your rooms in
//! the AWS region where the API request is processed. </p>
//! </li>
//! <li>
//! <p>
//! <a>UpdateRoom</a> — Updates a room’s configuration.</p>
//! </li>
//! </ul>
//!
//! <p>
//! <b>Tags Endpoints</b>
//! </p>
//! <ul>
//! <li>
//! <p>
//! <a>ListTagsForResource</a> — Gets information about AWS tags for the
//! specified ARN.</p>
//! </li>
//! <li>
//! <p>
//! <a>TagResource</a> — Adds or updates tags for the AWS resource with
//! the specified ARN.</p>
//! </li>
//! <li>
//! <p>
//! <a>UntagResource</a> — Removes tags from the resource with the
//! specified ARN.</p>
//! </li>
//! </ul>
//!
//!
//! <p>All the above are HTTP operations. There is a separate <i>messaging</i> API
//! for managing Chat resources; see the <a href="https://docs.aws.amazon.com/ivs/latest/chatmsgapireference/chat-messaging-api.html"> Amazon IVS Chat Messaging API
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
/// Generated accessors for nested fields
mod lens;
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
/// Paginators for the service
pub mod paginator;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::DateTime;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("ivschat", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
