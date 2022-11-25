use lunatic::{net::{TcpStream, TlsStream, ToSocketAddrs}, Mailbox};

#[lunatic::main]
fn main(_: Mailbox<()>) {
    println!("Hello world");
    let c = TcpStream::connect("127.0.0.1:1883").unwrap();

}
