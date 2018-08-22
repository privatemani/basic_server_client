use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::{BufRead, BufReader};

fn write_message(message: i32, mut stream: TcpStream) -> TcpStream {
    let number = message;
    let number = number + 1;
    let number_string = number.to_string();
    writeln!(stream, "{}", number_string).expect("failed to send message");
    stream
}
fn main() {
if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {

    let mut stream = BufReader::new(write_message(1, write_message(1, stream)));
    let mut buffer = String::new();
    stream.read_line(&mut buffer).expect("failed to read line");

    println!("{}", buffer);
    println!("Connected to the server!");
} else {
    println!("Couldn't connect to server...");
}
}
