mod event;
mod modules;
mod netconf;
mod parser;
mod socket;

use clap::Parser;
use log::{debug, error, info};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "SOCKET_PATH")]
    socket_path: Option<String>,
    #[arg(short, long, value_name = "MODULES_PATH")]
    modules_path: Option<String>,
}

fn handler_services_commit(input: &event::Data) {}

fn main() {
    let args = Cli::parse();
    env_logger::init();

    if args.socket_path.is_none() {
        error!("Error: socket path not provided");
        return;
    };

    let socket_path = args.socket_path.unwrap();

    if args.modules_path.is_none() {
        error!("Error: modules_path not provided");
        return;
    };

    let modules_path = args.modules_path.unwrap();
    let path = String::from(modules_path + "/*.so");
    let modules = modules::modules_find(&path);
    let stream = match socket::socket_create(&socket_path) {
        Ok(stream) => stream,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    // Enable subscriptions
    if let Err(e) = socket::socket_send(&stream, &netconf::NETCONF_CONTROLLER_TRANSACTION) {
        error!("Error: {}", e);
    }

    let response = match socket::socket_read_ok(&stream) {
        Ok(response) => {
            info!("Notifications enabled");
            response
        }
        Err(e) => {
            error!("Error reading from stream: {}", e);
            return;
        }
    };

    if let Err(e) = socket::socket_send(&stream, &netconf::NETCONF_SUBSCRIPTION_CREATE) {
        error!("Error: {}", e);
    }

    let response = match socket::socket_read_ok(&stream) {
        Ok(response) => {
            info!("Notifications enabled");
            response
        }
        Err(e) => {
            error!("Error reading from stream: {}", e);
            return;
        }
    };

    // Register events
    let mut event_handler = event::EventHandler::new();
    event_handler.register(
        "*<services-commit*>*</services-commit>*",
        Box::new(handler_services_commit),
    );

    loop {
        if let Ok(response) = socket::socket_read(&stream) {
            println!("{}", response);
        }
    }

    // Example: Call modules
    for module in modules {
        println!("{}", module);
        let _ = modules::module_call(&module, "test");
    }
}
