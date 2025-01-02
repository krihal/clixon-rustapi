use std::os::unix::net::UnixStream;
use std::io::Result;
use std::io::prelude::*;
use regex::Regex;

pub fn socket_create(sockpath: &str) -> Result<UnixStream> {
    UnixStream::connect(sockpath)
}

pub fn socket_send(mut stream: &UnixStream, data: &str) -> Result<()> {
    let frame_start = "\n#";
    let frame_end = "\n##\n";
    let frame_header = format!("{}{}\n", frame_start, data.len());
    let frame = format!("{}{}{}", frame_header, data, frame_end);
    
    println!("Sending {} bytes", data.len());
    
    stream.write_all(frame.as_bytes())?;

    Ok(())
}

pub fn socket_read(mut stream: &UnixStream) -> Result<String> {
    let mut buffer = String::new();
    let mut one_char = [0; 1];
    let mut _frame_size = "";

    let re = Regex::new(r"\n#(\d+)\n").unwrap();
    
    // Read one character at a time until we find the start of a frame and the regex matches
    loop {
        stream.read_exact(&mut one_char)?;
        buffer.push(one_char[0] as char);

        if let Some(captures) = re.captures(buffer.as_str()) { 
            _frame_size = captures.get(1).unwrap().as_str(); 
            break;
        }
    }

    // Read frame_size bytes from the stream
    let frame_size_int: usize = _frame_size.parse().unwrap();

    println!("Reading {} bytes", frame_size_int);

    let mut buffer = vec![0; frame_size_int];
    stream.read_exact(&mut buffer)?;

    Ok(String::from_utf8(buffer).unwrap())
}
