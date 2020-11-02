#[macro_use]
extern crate lazy_static;

use midir::{MidiInput, MidiOutput};
use std::error::Error;
use std::net::UdpSocket;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Opt {
    /// List devices
    #[structopt(short, long)]
    list: bool,

    #[structopt(default_value, short, long, required_if("list", "false"))]
    output: usize,

    #[structopt(default_value, short, long, required_if("list", "false"))]
    input: usize,

    server_addr: String,
}

lazy_static! {
    static ref SOCKET: UdpSocket =
        UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let midi_in = MidiInput::new("midir forwarding input")?;
    let midi_out = MidiOutput::new("midir forwarding output")?;
    let input_ports = midi_in.ports();
    let output_ports = midi_out.ports();

    if opt.list {
        println!("Available MIDI inputs:");
        for (i, p) in input_ports.iter().enumerate() {
            println!("[{}]\t {}", i, midi_in.port_name(p)?);
        }
        println!();
        println!("Available MIDI outputs:");
        for (i, p) in output_ports.iter().enumerate() {
            println!("[{}]\t {}", i, midi_out.port_name(p)?);
        }
        return Ok(());
    }

    let in_port = input_ports.get(opt.input).expect("invalid input device");
    let out_port = output_ports.get(opt.output).expect("invalid output device");

    SOCKET.connect(&opt.server_addr)?;

    let _input_connection = midi_in.connect(
        &in_port,
        "networked-midi",
        move |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
            SOCKET.send(message).expect("error sending message");
        },
        (),
    )?;

    let mut output_connection = midi_out.connect(&out_port, "networked-midi")?;

    loop {
        let mut buf = [0; 3];
        SOCKET.recv_from(&mut buf)?;
        println!("Received message: {:?}", buf);
        output_connection.send(&buf)?;
    }
}
