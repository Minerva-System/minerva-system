use chrono;
use std::net::SocketAddr;

pub fn print(addr: SocketAddr, requestor: String, tenant: String, msg: &str) {
    let msg = msg.to_string();
    let time = chrono::offset::Local::now();
    tokio::spawn(async move {
        println!("{} :: {}@{:?} :: {}>{}", time, requestor, addr, tenant, msg);
    });
}
