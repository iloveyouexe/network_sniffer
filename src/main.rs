use clap::{App, Arg};
use pnet::datalink::{self, Channel::Ethernet};

fn main() {
    env_logger::init();

    println!("Initializing network sniffer...");

    let matches = App::new("Network Sniffer")
        .version("0.1.0")
        .author("Your Name")
        .about("CLI tool for monitoring network traffic")
        .arg(
            Arg::with_name("interface")
                .short('i')
                .long("interface")
                .value_name("INTERFACE")
                .help("Sets the network interface to listen on")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("filter")
                .short('f')
                .long("filter")
                .value_name("FILTER")
                .help("Sets a filter to apply on the captured packets")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help("Sets the number of packets to capture")
                .takes_value(true)
                .default_value("10"), // Capture 10 packets by default
        )
        .get_matches();
}
