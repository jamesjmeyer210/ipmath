use std::path::PathBuf;
use std::str::FromStr;
use clap::{Parser};
use ipmath_cli::Format;
use ipmath_core::IpMath;
use ipmath_core::net::{IpAddress, IpFormat};

#[derive(Parser)]
#[clap(name = "ipmath", author = "James Meyer")]
struct Cli {
    #[arg(short, long, value_name = "IP ADDRESS")]
    convert: Option<String>,
    #[arg(long)]
    format_in: Option<Format>,
    #[arg(short, long, value_name = "FILE")]
    out: Option<PathBuf>,
    #[arg(long)]
    format_out: Option<Format>,
}

fn main() {
    let cli = Cli::parse();

    if cli.convert.is_some() {
        let ip = cli.convert.unwrap();
        let r = IpMath::convert(&ip, cli.format_in.map(|x|x.into()), cli.format_out.map(|x|x.into()));
        match r {
            Ok(x) => println!("{x}"),
            Err(e) => println!("{e}")
        }
    }
}
