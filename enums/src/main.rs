mod enum_examples;

#[derive(Debug)]
enum IpAddrKind {
    // To define all enumartion types possible for a class of data --> eg: Types of IP adddr. - IPv4 & IPv6
    V4,
    V6,
}

struct IpAddress {
    address: String,
    kind: IpAddrKind,
}

impl IpAddress {
    fn new(address: &str) -> Self {
        // associated func.
        Self {
            address: address.to_string(),
            kind: IpAddrKind::V4, // can be made dynamic according to string via validations
        }
    }
}

/* CLEANER REPRESENTATION: Using ONLY Structs */
#[derive(Debug)]
enum IpAddressKind2 {
    V4(u8, u8, u8, u8), // 4-octet
    V6(String),
}

fn main() {
    naive_route("1.2.3.4", IpAddrKind::V4);
    naive_route("1.2.3.4.5.6", IpAddrKind::V6);

    // Struct implementation
    let address = IpAddress::new("10.111.23.90");
    struct_route(address);

    // Cleaner implementation
    let home = IpAddressKind2::V4(127, 0, 0, 1);
    let loopback = IpAddressKind2::V6(String::from("::1"));
    enum_route(home);
    enum_route(loopback);

    // Option<T> enum
    enum_examples::option_enum();
}

fn naive_route(ip: &str, kind: IpAddrKind) {
    println!("{ip}: {kind:?}");
}

fn struct_route(ip: IpAddress) {
    println!("{}: {:?}", ip.address, ip.kind);
}

fn enum_route(ip: IpAddressKind2) {
    println!("Routing request to: {:?}", ip)
}
