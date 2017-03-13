use std::env;
pub mod ip;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();
    match args.len() {
            2 => {

                let address_parts: Vec<&str> = args[1].split('/').collect();
                if address_parts.len() == 2 {
                    if ip::is_valid(address_parts[0]) && address_parts[1].parse::<u8>().unwrap() < 33 {
                        let mask_from_cidr = match ip::make_mask_from_cidr(address_parts[1].parse::<u8>().unwrap()) {
                            Ok(m) => m,
                            Err(_) => {
                                println!("Your CIDR is too high");
                                process::exit(1);
                            }
                        };
                        ip::guess_gateway(address_parts[0], mask_from_cidr);
                    } else {
                        println!("Check your IP address again and make sure it is correct.");
                    }
                } else {
                    panic!("There should only be 1 slash, an IP address first then a CIDR notation (1-32) after the slash");
                }
            },
            3 => {
                if ip::is_valid(&args[1]) {
                    ip::guess_gateway(&args[1], ip::make_mask_from_string(&args[2]).unwrap());
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
