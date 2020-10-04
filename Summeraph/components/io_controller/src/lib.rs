pub mod storage;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use mio::net::TcpListener;
    use std::net::{SocketAddrV4, Ipv4Addr, SocketAddr, IpAddr};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
