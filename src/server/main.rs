use std::collections::HashSet;
use std::error::Error;
use std::net::{SocketAddr, UdpSocket};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Opt {
    #[structopt(default_value = "9000", short, long, required_if("list", "false"))]
    port: usize,
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let socket = UdpSocket::bind(format!("{}:{}", "127.0.0.1", opt.port.to_string()))?;
    let mut clients: HashSet<SocketAddr> = HashSet::new();

    loop {
        let mut buf = [0; 3];
        let (amt, src) = socket.recv_from(&mut buf)?;
        clients.insert(src);
        println!("{:?}", &buf[0..amt]);
        for client in &clients {
            if client == &src {
                continue;
            }
            socket.send_to(&buf, client)?;
        }
    }
}
