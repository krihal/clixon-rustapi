mod socket;
mod parser;
mod netconf;
mod modules;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "SOCKET_PATH")]
    socket_path: Option<String>,
}

fn main() -> ! {
    let args = Cli::parse();

    let socket_path = match args.socket_path {
        Some(socket_path) => socket_path,
        None => {
            println!("Error: socket path not provided");
            return;
        }
    };

    let stream = match socket::socket_create(&socket_path) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };   

    // Enable subscriptions
    if let Err(e) = socket::socket_send(&stream, &netconf::NETCONF_SUBSCRIPTION_CREATE) {
        println!("Error: {}", e);	
    }

    let response = match socket::socket_read_ok(&stream) {
        Ok(response) => response,
        Err(e) => {
            println!("Error reading from stream: {}", e);
            return;
        }
    };

    // Eternal loop which listens for messagaes
    loop {
        ;
    }
}
