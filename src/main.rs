use std::env;
extern crate gateway;

fn main() {

    let args: Vec<String> = env::args().collect();
    match args.len() {
            2 => {
                    let address_parts: Vec<&str> = args[1].split('/').collect();
                    if address_parts.len() == 2 {
                        if (gateway::ip::is_valid(address_parts[0]) && address_parts[1].to_string().parse::<u8>().unwrap() < 33) {
                                gateway::ip::guess_gateway(address_parts[0], gateway::ip::make_mask_from_cidr(address_parts[1].to_string().parse::<u8>().unwrap()))
                        } else {
                            println!("Check your IP address again and make sure it is correct.");
                        }
                    } else {
                        panic!("There should only be 1 slash, an IP address first then a CIDR notation (1-32) after the slash");
                    }
            },
            3 => {
                if gateway::ip::is_valid(&args[1]) {
                    gateway::ip::guess_gateway(&args[1], gateway::ip::make_mask_from_string(&args[2]));
                } else {
                    println!("Host address should look like this: 192.168.0.1")
                }
            },
            _ => {
println!("Gateway Guesser
Returns expected Gateway address for a given IP and subnet
Usage:
gateway IP/mask CIDR notation or
gateway IP mask
Examples:
gateway 192.168.0.2/24
gateway 192.168.0.2 255.255.255.0")
            }
    }
}
