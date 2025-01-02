mod socket;
mod parser;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]

struct Cli {
    #[arg(short, long, value_name = "SOCKET_PATH")]
    socket_path: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let socket_path = match args.socket_path {
        Some(socket_path) => socket_path,
        None => {
            println!("Error: socket path not provided");
            return;
        }
    };

    let hello_str = "<rpc xmlns=\"urn:ietf:params:xml:ns:netconf:base:1.0\" cl:username=\"debian\" xmlns:cl=\"http://clicon.org/lib\" xmlns:nc=\"urn:ietf:params:xml:ns:netconf:base:1.0\" message-id=\"42\"><get-config><source><running/></source><nc:filter nc:type=\"xpath\" nc:select=\"/\"/></get-config></rpc>";

    let stream = match socket::socket_create(&socket_path) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };   

    if let Err(e) = socket::socket_send(&stream, hello_str) {
        println!("Error: {}", e);	
    }

    let response = match socket::socket_read(&stream) {
        Ok(response) => response,
        Err(_) => {
            println!("Error reading from stream");
            return;
        }
    };

    let root = match parser::parse_string(&response) {
        Ok(root) => root,
        Err(_) => {
            println!("Error parsing XML");
            return;
        }
    };
}
