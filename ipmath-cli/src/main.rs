use std::path::PathBuf;
use std::str::FromStr;
use clap::{Parser};
use ipmath_core::net::IpAddress;

#[derive(Parser)]
#[clap(name = "ipmath", author = "James Meyer")]
struct Cli {
    #[arg(short, long, value_name = "IP ADDRESS")]
    convert: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    out: Option<PathBuf>
}

fn main() {
    let cli = Cli::parse();

    if cli.convert.is_some() {
        let c = cli.convert.unwrap();
        let ip = IpAddress::from_str(&c);
        if ip.is_err() {
            println!("Failed to parse {c}");
        }

        let ip = ip.unwrap();
        println!("IP Address: {ip}")
    }
}
