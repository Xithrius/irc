//! Support for the IRC protocol using Tokio.

#![warn(missing_docs, clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::unused_self,
    clippy::future_not_send,
    clippy::suboptimal_flops,
    clippy::enum_glob_use,
    clippy::too_many_lines,
    clippy::many_single_char_names,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::cognitive_complexity,
    clippy::missing_const_for_fn
)]

pub mod caps;
pub mod chan;
pub mod colors;
pub mod command;
pub mod error;
#[cfg(feature = "tokio")]
pub mod irc;
#[cfg(feature = "tokio")]
pub mod line;
pub mod message;
pub mod mode;
pub mod prefix;
pub mod response;

pub use self::caps::{Capability, NegotiationVersion};
pub use self::chan::ChannelExt;
pub use self::colors::FormattedStringExt;
pub use self::command::{BatchSubCommand, CapSubCommand, Command};
#[cfg(feature = "tokio")]
pub use self::irc::IrcCodec;
pub use self::message::Message;
pub use self::mode::{ChannelMode, Mode, UserMode};
pub use self::prefix::Prefix;
pub use self::response::Response;
