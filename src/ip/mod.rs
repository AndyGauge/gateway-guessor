pub fn is_valid(address: &str) -> bool {
    let octets = address.split(".");
    let mut octet_count: u8 = 0;
    for byte in octets {
        match byte.to_string().parse::<u8>() {
        Err(e) => return false,
        Ok(n) => octet_count += 1,
        }
        if octet_count == 4 {
            return true
        }
    }
    return false
}

pub fn make_mask_from_cidr(cidr: u8) -> u32 {
    0xffffffff_u32 & !((0xffffffff_u64 >> cidr) as u32)
}

pub fn make_mask_from_string(address: &str) -> u32 {
    let octets = address.split(".");
    let mut octet_count: u8 = 3;
    let mut mask: u32 = 0;
    for byte in octets {
        let ibyte = byte.to_string().parse::<u8>().unwrap();
        mask += 256u32.pow(octet_count as u32) * ibyte as u32;
        if octet_count > 0 {
            octet_count -= 1;
        }
    }
    return mask
}

pub fn make_ip_from_mask(mask: u32) -> String {
    let oct1 = ((mask & 0xFF000000) / 0xFFFFFF) as u8;
    let oct2 = ((mask & 0xFF0000) / 0xFFFF) as u8;
    let oct3 = ((mask & 0xFF00) /0xFF) as u8;
    let oct4 = (mask & 0xFF) as u8;
    format!("{}.{}.{}.{}", oct1, oct2, oct3, oct4)
}
pub fn guess_gateway(host: &str, subnet_mask: u32) {
    let hostu32 = make_mask_from_string(host);
    let network = hostu32 & subnet_mask;
    let broadcast = hostu32 | !subnet_mask;
    println!("The network address for this host is: {}", make_ip_from_mask(network));
    println!("The broadcast address for this host is: {}", make_ip_from_mask(broadcast));
    println!("I'm guessing the gateway is {} or {}", make_ip_from_mask(network+1), make_ip_from_mask(broadcast-1));
}
