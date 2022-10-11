use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum DispatchError {
    #[snafu(display("{}: Unable to connect to {}.", consumer_name, service_name))]
    Connection {
        consumer_name: String,
        service_name: String,
    },
    #[snafu(display("{}: Error while receiving delivered message.", consumer_name))]
    Delivery { consumer_name: String },
    #[snafu(display("{}: Unable to ACK delivery.", consumer_name))]
    Ack { consumer_name: String },
}
