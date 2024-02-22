use std::env::args;

fn main() {
    let my_args: Vec<String> = args().collect();

    let ipv6_address = my_args[1].to_string();

    let hextets: [&str; 8] = ipv6_address
        .split(":")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();
    for hextet in hextets {
        println!("{hextet}")
    }
}

// fn parse_string(target_string: String) {
//     let mut counter = 0;
//     let mut group = 0;
//     println!("");
//     for char in target_string.chars() {
//         counter += 1;
//         print!("{char}");
//         if counter == 4 {
//             group += 1;
//             if group < 8 {
//                 print!(":");
//             }
//             counter = 0;
//         }
//     }
// }