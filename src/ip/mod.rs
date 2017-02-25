#[derive(Debug, PartialEq)]
struct NetworkAddr<T>(T);

#[derive(Debug, PartialEq)]
struct BroadcastAddr<T>(T);

type NetworkValues = (NetworkAddr<u32>, BroadcastAddr<u32>);

pub fn is_valid(address: &str) -> bool {
    let octets = address.split(".");
    let mut octet_count: u8 = 0;
    for byte in octets {
        match byte.parse::<u8>() {
            Err(_) => return false,
            Ok(_)  => octet_count += 1,
        }
        if octet_count > 4 {
            return false;
        }
    }
    return octet_count == 4
}


pub fn make_mask_from_cidr(cidr: u8) -> Result<u32, ()> {
    if cidr > 32 {
        return Err(());
    }
    let mask = 0xffffffff_u32 & !((0xffffffff_u64 >> cidr) as u32);   
    return Ok(mask)

}

pub fn make_mask_from_string(address: &str) -> u32 {
    let octets = address.split(".");
    let mut octet_count: u8 = 3;
    let mut mask: u32 = 0;
    for byte in octets {
        let ibyte = byte.parse::<u8>().unwrap();
        mask += 256u32.pow(octet_count as u32) * ibyte as u32;
        if octet_count > 0 {
            octet_count -= 1;
        }
    }
    return mask
}

pub fn make_ip_from_mask(mask: u32) -> String {
    let oct1 = ((mask & 0xFF000000) >> 24)  as u8;
    let oct2 = ((mask & 0xFF0000  ) >> 16)  as u8;
    let oct3 = ((mask & 0xFF00    ) >> 8)   as u8;
    let oct4 = ( mask & 0xFF      )         as u8;

    format!("{}.{}.{}.{}", oct1, oct2, oct3, oct4)
}

fn calc_network_values(host: &str, cidr: u32) -> NetworkValues {
    let hostu32 = make_mask_from_string(host);

    (NetworkAddr(hostu32 & cidr), BroadcastAddr(hostu32 | !cidr))
}

pub fn guess_gateway(host: &str, subnet_mask: u32) {

    let (NetworkAddr(network), BroadcastAddr(broadcast)) = calc_network_values(host, subnet_mask);
    println!("The network address for this host is: {}", make_ip_from_mask(network));
    println!("The broadcast address for this host is: {}", make_ip_from_mask(broadcast));
    println!("I'm guessing the gateway is {} or {}", make_ip_from_mask(network+1), make_ip_from_mask(broadcast-1));
    
}

#[cfg(test)]
mod tests {
    use super::{
        make_ip_from_mask,
        make_mask_from_string,
        make_mask_from_cidr,
        calc_network_values,
        NetworkAddr,
        BroadcastAddr
    };

    macro_rules! ipv4 {
        ($a:expr, $b:expr, $c:expr, $d:expr) => {
            (($a << 24) | ($b << 16) | ($c << 8) | $d) as u32
        }
    }

    #[test]
    fn test_ip_from_mask() {
        let mask = ipv4!(192,168,255,255);
        assert_eq!("192.168.255.255", make_ip_from_mask(mask));
    }

    #[test]
    fn test_mask_from_string() {
        let mask = ipv4!(192,168,16,1);
        assert_eq!(mask, make_mask_from_string("192.168.16.1"));
    }

    #[test]
    fn test_network_values() {
        let network = ipv4!(192,168,0,0);
        let broadcast = ipv4!(192,168,255,255);
        let mask = make_mask_from_cidr(16).unwrap();

        assert_eq!((NetworkAddr(network), BroadcastAddr(broadcast)), 
            calc_network_values("192.168.0.15", mask));
    }
}

