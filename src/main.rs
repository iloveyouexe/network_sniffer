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

    let interface_name = matches.value_of("interface").unwrap_or("default_interface_name");
    let interface = datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("Error finding interface");

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Error creating datalink channel: {}", e),
    };
    let mut packet_count = 0;
    let total_packets_to_capture = matches.value_of("count").unwrap().parse::<i32>().unwrap();

    while packet_count < total_packets_to_capture {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                // Process the packet (e.g., print out its contents)
                println!("Received packet: {:?}", packet);
                packet_count += 1;
            }
            Err(e) => {
                // Handle the error...
            }
        }
    }
}
