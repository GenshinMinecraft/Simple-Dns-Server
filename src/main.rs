use dns_server::DnsRecord;
use std::env;
use std::net::IpAddr;

fn main() {

    let port: String = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Error: Missing Port");
        std::process::exit(1);
    });

    let domain: String = env::args().nth(2).unwrap_or_else(|| {
        eprintln!("Error: Missing Domain");
        std::process::exit(1);
    });

    let ip: String = env::args().nth(3).unwrap_or_else(|| {
        eprintln!("Error: Missing IP");
        std::process::exit(1);
    });

    let mut records: Vec<DnsRecord> = vec![];

    match ip.as_str().parse::<IpAddr>() {
        Ok(IpAddr::V4(_)) => {
            records.push(DnsRecord::new_a(&domain, &ip).unwrap());
        },
        Ok(IpAddr::V6(_)) => {
            records.push(DnsRecord::new_aaaa(&domain, &ip).unwrap());
        },
        Err(_) => eprintln!("Invalid IP address"),
    }

    let _server = dns_server::Builder::new_port(port.parse().unwrap())
        .unwrap()
        .serve_static(&records)
        .unwrap();

}
