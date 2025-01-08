mod socket;
mod parser;
mod netconf;
mod modules;
mod event;

use clap::Parser;
use log::{info, debug, error};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "SOCKET_PATH")]
    socket_path: Option<String>,
}

fn handler_services_commit(input: &event::Data) {

}

fn main() {
    let args = Cli::parse();
    env_logger::init();

    let socket_path = match args.socket_path {
        Some(socket_path) => socket_path,
        None => {
            error!("Error: socket path not provided");
            return;
        }
    };

    let stream = match socket::socket_create(&socket_path) {
        Ok(stream) => stream,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    // Enable subscriptions
    if let Err(e) = socket::socket_send(&stream, &netconf::NETCONF_SUBSCRIPTION_CREATE) {
        error!("Error: {}", e);
    }

    let response = match socket::socket_read_ok(&stream) {
        Ok(response) => response,
        Err(e) => {
            error!("Error reading from stream: {}", e);
            return;
        }
    };

    // Register events
    let mut event_handler = event::EventHandler::new();
    event_handler.register("*<services-commit*>*</services-commit>*", Box::new(handler_services_commit));

    // Eternal loop which listens for messagaes
    loop {
    	info!("Listening for messages...");
    	let data = match socket::socket_read(&stream) {
    		Ok(data) => data,
    		Err(e) => {
    			error!("Error reading from stream: {}", e);
    			continue;
    		}
    	};
    }
}
