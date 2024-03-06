use std::env::args;

fn main(){
    // let subnet_args: Vec<String> = args().collect();
    // for (i, arg) in subnet_args.iter().enumerate() {}
    for (i, arg) in args().enumerate(){
        // println!("Arg {i}: {arg}");

        if i == 1 {
            calculate_subnet(arg.to_string());
        }
    }
}

fn calculate_subnet(target_subnet: String) {
    //iterate over a split string to seperate main octets from /
    // let parts = target_subnet.split("/");
    // for part in parts {
    //     println!("Subnet part: {part}");
    // }
    
    //split strings into named variables
    let [target_ip, cidr]: [&str; 2] = target_subnet
        .split("/")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();

    let octet_strings: [&str; 4] = target_ip
        .split(".")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();

    // For prep of use of exponents   
    let exponential_base: u32 = 2;
    // Parse cidr into u32
    let cidr = cidr.parse::<u32>().unwrap();
    let mut subnet_bit_size = 256 / (exponential_base.pow(cidr % 8));
    //For exception because of my implementation
    if subnet_bit_size == 256 { subnet_bit_size = 1; }
    
    let octet_layer: u32 = (cidr - 1) / 8;
    if octet_layer >= 5 {
        println!("Use a proper ipv4 address please");
        return;
    }

    let mut temp_counter: u32 = 0;
    //Parse octet_layer for use in array
    let octet_layer = octet_layer as usize;
    let octet_value = octet_strings[octet_layer].parse::<u32>().unwrap();
    while temp_counter <= octet_value {
        temp_counter += subnet_bit_size;
    }
    temp_counter -= subnet_bit_size;

    //Now Concatinate together network Id

    let mut network_id :String = "".to_string();
    for x in 0..=3 {
        if x == octet_layer {
            network_id.push_str(&temp_counter.to_string());
        } else {
            network_id.push_str(octet_strings[x]);
        }
        if x != 3 {
            network_id.push_str(".");
        }
    }

    temp_counter += 1;
    let mut first_host :String = "".to_string();
    for x in 0..=3 {
        if x == octet_layer {
            first_host.push_str(&temp_counter.to_string());
        } else {
            first_host.push_str(octet_strings[x]);
        }
        if x != 3 {
            first_host.push_str(".");
        }
    }

    temp_counter += subnet_bit_size -2;
    let mut broadcast_ip :String = "".to_string();
    for x in 0..=3 {
        if x == octet_layer {
            broadcast_ip.push_str(&temp_counter.to_string());
        } else {
            broadcast_ip.push_str(octet_strings[x]);
        }
        if x != 3 {
            broadcast_ip.push_str(".");
        }
    }

    temp_counter -= 1;
    let mut last_host :String = "".to_string();
    for x in 0..=3 {
        if x == octet_layer {
            last_host.push_str(&temp_counter.to_string());
        } else {
            last_host.push_str(octet_strings[x]);
        }
        if x != 3 {
            last_host.push_str(".");
        }
    }

    temp_counter += 2;
    let mut next_network :String = "".to_string();
    for x in 0..=3 {
        if x == octet_layer {
            next_network.push_str(&temp_counter.to_string());
        } else {
            next_network.push_str(octet_strings[x]);
        }
        if x != 3 {
            next_network.push_str(".");
        }
    }

    let ip_addresses = subnet_bit_size;
    let usable_ip_addresses = subnet_bit_size -2;

    let mut subnet_mask :String = "".to_string();
    for x in 0..=3 {
        if x == 3 {
            let mask_value = 256 - subnet_bit_size;
            subnet_mask.push_str(&mask_value.to_string());
        }
        else {
            subnet_mask.push_str("255.")
        }
    }

    //First way of getting an int from &str
    // let parsed_octet: i32 = octet.parse().unwrap();
    //Second way of getting an int from &str
    // let parsed_octet = octet.parse::<i32>().unwrap();

    println!("Network ID: {network_id}");
    println!("Broadcast IP: {broadcast_ip}");
    println!("First Host: {first_host}");
    println!("Last Host: {last_host}");
    println!("Next Network: {next_network}");
    println!("Number of IP Addresses: {ip_addresses}");
    println!("Number of Usable IP Addresses: {usable_ip_addresses}");
    println!("CIDR/Subnet Mask: {subnet_mask}");
}