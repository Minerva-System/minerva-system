use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum DispatchError {
    #[snafu(display("{}: Unable to connect to {}.", consumer_name, service_name))]
    ConnectionError {
        consumer_name: String,
        service_name: String,
    },
    #[snafu(display("{}: Error while receiving delivered message.", consumer_name))]
    DeliveryError { consumer_name: String },
    #[snafu(display("{}: Unable to ACK delivery.", consumer_name))]
    AckError { consumer_name: String },
}
