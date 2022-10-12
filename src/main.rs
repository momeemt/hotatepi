use std::{net::{TcpStream}, io::{BufReader, BufWriter, BufRead, Write, Read}, str::from_utf8};
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
    let mut header = Vec::new();
    let mut content_length = 0;

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).expect("Failed to read");
        if reader.buffer().is_empty() || line.is_empty() {
            break
        }
        if line.starts_with("Content-Length") {
            content_length = line.split(':').collect::<Vec<&str>>()[1].trim().parse().unwrap();
        }
        header.push(line);
    }

    println!("{}", content_length);
    println!("{:?}", header);

    let mut body = String::new();

    if 0 < content_length {
        let mut c = Vec::with_capacity(content_length);
        reader.read(&mut c);
        body = String::from_utf8(c).unwrap();
    }

    println!("{}", body);

    return header
}

fn write_to_tcp (writer: &mut BufWriter<&TcpStream>, data: &str) {
    let msg = format!("MESSAGE: {}\n", data);
    writer.write(msg.as_bytes()).expect("Failed to send");
    writer.flush().unwrap();
}
