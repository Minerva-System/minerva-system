//! This module describes errors that could arise in this module.

use snafu::Snafu;

/// Describes an error pertaining a message dispatch. These kinds of errors
/// may arise before or after trying to process the message.
#[derive(Debug, Snafu)]
pub enum DispatchError {
    /// Describes a connection error while attempting to connect this consumer
    /// to a given queue or channel.
    #[snafu(display("{}: Unable to connect to {}.", consumer_name, service_name))]
    Connection {
        consumer_name: String,
        service_name: String,
    },
    /// Describes an error while receiving a message from the queue or channel.
    #[snafu(display("{}: Error while receiving delivered message.", consumer_name))]
    Delivery { consumer_name: String },
    /// Describes an error while trying to send back and acknowledgement
    /// signal to the queue or channel.
    #[snafu(display("{}: Unable to ACK delivery.", consumer_name))]
    Ack { consumer_name: String },
}
