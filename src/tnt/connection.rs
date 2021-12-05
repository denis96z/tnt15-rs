use tokio::{io::BufWriter, net::TcpStream};

pub struct Connection {
    writer: BufWriter<TcpStream>,

    req_id: i32,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            writer: BufWriter::new(socket),
            req_id: 0,
        }
    }

    fn next_req_id(&mut self) -> i32 {
        self.req_id += 1;
        self.req_id
    }
}
