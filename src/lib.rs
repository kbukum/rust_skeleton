extern crate bytes;

use std::net::SocketAddr;

#[derive(Debug)]
pub struct Props {
    pub host: String,
    pub port: i32
}


impl  Props {
    fn to_address(&self) -> SocketAddr {
        let address = format!("{}:{}", self.host, self.port);
        address.parse().expect("Unable to parse socket address")
    }
}

pub struct Server;

impl Server {
    pub fn serve(props: Props) {
        let address = props.to_address();
    }
}
