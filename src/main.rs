use std::{net::{TcpStream}, io::{BufReader, BufWriter, BufRead, Write}};
use std::{net::{TcpListener}};

fn main() -> std::io::Result<()> {
    println!("start >>>");

    let listener = TcpListener::bind("localhost:80")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    println!("<<< end");

    Ok(())
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    for str in read_from_tcp(&mut reader) {
        println!("{}", str);
    }
}

fn read_from_tcp (reader: &mut BufReader<&TcpStream>) -> Vec<String> {
    let mut msgs = Vec::new();
    loop {
        let mut msg = String::new();
        reader.read_line(&mut msg).expect("Failed to read");
        if reader.buffer().is_empty() || msg.is_empty() {
            break
        }
        msgs.push(msg);
    }
    return msgs;
}

fn write_to_tcp (writer: &mut BufWriter<&TcpStream>, data: &str) {
    let msg = format!("MESSAGE: {}\n", data);
    writer.write(msg.as_bytes()).expect("Failed to send");
    writer.flush().unwrap();
}
