use std::error::Error;
use tokio::{
    io,
    net::{TcpStream, ToSocketAddrs},
};

use super::Connection;

pub struct Client {
    connection: Connection,
}

pub async fn connect<T: ToSocketAddrs>(addr: T) -> io::Result<Client> {
    let socket = TcpStream::connect(addr).await?;
    Ok(Client {
        connection: Connection::new(socket),
    })
}

impl Client {}
