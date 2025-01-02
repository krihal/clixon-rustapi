use std::os::unix::net::UnixStream;
use std::io::Result;
use std::io::prelude::*;
use regex::Regex;

pub fn socket_create(sockpath: &str) -> Result<UnixStream> {
    let stream = UnixStream::connect(sockpath);

    return stream;
}

pub fn socket_send(mut stream: &UnixStream, data: &str) -> Result<()> {
    let frame_start = "\n#";
    let frame_end = "\n##\n";
    let frame_header = format!("{}{}\n", frame_start, data.len());
    let frame = format!("{}{}{}", frame_header, data, frame_end);
    
    println!("Sending {} bytes", data.len());
    
    if let Err(e) = stream.write_all(frame.as_bytes()) {
        return Err(e);
    }

    return Ok(());
}

pub fn socket_read(mut stream: &UnixStream) -> Result<String> {
    let mut buffer = String::new();
    let mut one_char = [0; 1];
    let mut frame_size = "";

    let re = Regex::new(r"\n#(\d+)\n").unwrap();
    
    // Read one character at a time until we find the start of a frame and the regex matches
    loop {
        stream.read(&mut one_char)?;
        buffer.push(one_char[0] as char);

        if let Some(captures) = re.captures(buffer.as_str()) { 
            frame_size = captures.get(1).unwrap().as_str(); 
            break;
        }
    }

    // Read frame_size bytes from the stream
    let frame_size_int: usize = frame_size.parse().unwrap();

    println!("Reading {} bytes", frame_size_int);

    buffer = String::new();

    while buffer.len() < frame_size_int {
        stream.read(&mut one_char)?;
        buffer.push(one_char[0] as char);
    }

    // Return
    return Ok(buffer);
}
