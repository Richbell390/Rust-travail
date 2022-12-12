use std::io::{Read, Write};
use std::net::TcpStream;


fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream) => {
             /* Message 1*/
        let message = "\"Hello\"";

        let buf = message.as_bytes();
        let n = buf.len() as u32;
        let mut buf_n = n.to_be_bytes();
        stream.write(&buf_n).unwrap();
        stream.write(&buf).unwrap();

        stream.read_exact(&mut buf_n).unwrap();
        let n = u32::from_be_bytes(buf_n);
        let mut buf = Vec::<u8>::new();
        buf.resize(n as usize, 0);
        let s = stream.read(&mut buf).expect("Cannot read");
        let msg = String::from_utf8_lossy(&buf);

        println!("Receive message {msg}");


        /* Message 2*/
        let message2 = "{\"Subscribe\":{\"name\":\"Richbell\"}}";

        let buf2 = message2.as_bytes();
        let n2 = buf2.len() as u32;
        let mut buf_n2 = n2.to_be_bytes();
        stream.write(&buf_n2).unwrap();
        stream.write(&buf2).unwrap();

        stream.read_exact(&mut buf_n2).unwrap();
        let n2 = u32::from_be_bytes(buf_n2);
        let mut buf2 = Vec::<u8>::new();
        buf2.resize(n2 as usize, 0);
        let s = stream.read(&mut buf2).expect("Cannot read");
        let msg2 = String::from_utf8_lossy(&buf2);

        println!("Receive message {msg2}");


        }
        Err(err) => { panic!("ERROR: {err}") }
    }
}
