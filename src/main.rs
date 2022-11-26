use std::time::Duration;

use lunatic::Mailbox;
use mqtt::control::ConnectReturnCode;
use mqtt::packet::{
    ConnackPacket, ConnectPacket, DisconnectPacket, PingrespPacket, VariablePacket, PingreqPacket,
};
use mqtt::{Decodable, Encodable};

#[lunatic::main]
fn main(_: Mailbox<()>) {
    println!("Lunatic mqtt wasm: starting…");
    const HEARTBEAT: u16 = 60;

    let mut stream = lunatic::net::TcpStream::connect("127.0.0.1:1883").unwrap();
    // let stream = net::TlsStream::connect("127.0.0.1:8883").unwrap();

    let mut connect = ConnectPacket::new("37365bd3-3a50-465d-acdc-8ba432c4ecfb");
    connect.set_clean_session(true);
    connect.set_keep_alive(HEARTBEAT*2);

    connect
        .encode(&mut stream)
        .expect("Failed to send ConnectPacket");
    let connack = ConnackPacket::decode(&mut stream).expect("Expected ConnackPacket");
    if connack.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
        panic!(
            "Failed to connect to server, return code {:?}",
            connack.connect_return_code()
        );
    }

    let pinger = lunatic::spawn_link!(|stream = {stream.clone()}| {
        loop {
            lunatic::sleep(Duration::from_secs((HEARTBEAT).into()));
            PingreqPacket::new().encode(&mut stream).expect("Ping request sending failed");
        }
    });

    println!("Loop started");
    loop {
        let packet = VariablePacket::decode(&mut stream)
            .expect("Decoding a mqtt packet failed");
        match packet {
            VariablePacket::PingreqPacket(..) => {
                let resp = PingrespPacket::new();
                resp.encode(&mut stream).expect("Ping response sending failed");
            }
            VariablePacket::DisconnectPacket(..) => {
                break;
            }
            _ => {},
        };
    }
    pinger.kill();
    DisconnectPacket::default()
        .encode(&mut stream)
        .expect("Failed to send DisconnectPacket");

}
