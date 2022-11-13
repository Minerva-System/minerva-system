//! This module wraps functions related to logging stuff to screen.
//! These are non-audit logs, which should be related to normal operation of the
//! system, mainly captured by whatever deploy method is used.

use std::net::SocketAddr;

/// Prints a log line to the screen. Depends on an address of the service
/// accessing the server, the username of the service requestor, whatever tenant
/// is being accessed, and a message to be displayed.
#[deprecated(
    since = "0.3.0",
    note = "Use the `log` crate and `format` function instead"
)]
pub fn print(addr: SocketAddr, requestor: String, tenant: String, msg: &str) {
    let msg = msg.to_string();
    println!("{}@{:?} :: {}>{}", requestor, addr, tenant, msg);
}

/// Formats a log line with the default expected text. Depends on an address of
/// the service accessing the server, the username of the service requestor,
/// whatever tenant is being accessed, and a message to be displayed.
pub fn format(addr: SocketAddr, requestor: &str, tenant: &str, msg: &str) -> String {
    format!("{} ({}.{}@{})", msg, requestor, tenant, addr)
}
