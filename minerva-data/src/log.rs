//! This module wraps functions related to logging stuff to screen.
//! These are non-audit logs, which should be related to normal operation of the
//! system, mainly captured by whatever deploy method is used.

use chrono;
use std::net::SocketAddr;

/// Prints a log line to the screen. Depends on an address of the service
/// accessing the server, the username of the service requestor, whatever tenant
/// is being accessed, and a message to be displayed.
pub fn print(addr: SocketAddr, requestor: String, tenant: String, msg: &str) {
    let msg = msg.to_string();
    let time = chrono::offset::Local::now();
    tokio::spawn(async move {
        println!("{} :: {}@{:?} :: {}>{}", time, requestor, addr, tenant, msg);
    });
}
