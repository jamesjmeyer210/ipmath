use clap::{Arg, value_parser};
use clap::{Command};
use ipmath_cli::Format;
use ipmath_core::IpMath;

fn main() {

    let cmd = Command::new("ipmath")
        .author("James Meyer, jamesjmeyer@gmail.com")
        .version("0.1.0")
        .arg(Arg::new("IGNORE ERRORS")
            .short('I')
            .long("ignore-errors"))
        .arg(Arg::new("OUTPUT FILE")
            .short('o')
            .long("output"))
        .subcommand(Command::new("convert")
            .args([
                Arg::new("IP").required(true),
                Arg::new("INPUT_FORMAT")
                    .long("format-in")
                    .value_parser(value_parser!(Format)),
                Arg::new("OUTPUT_FORMAT")
                    .long("format-out")
                    .value_parser(value_parser!(Format))
            ])
        );

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("convert", sub_matches)) => {
            let ip = sub_matches.get_one::<String>("IP").unwrap();
            let format_in = sub_matches.get_one::<Format>("INPUT_FORMAT").map(|x|x.to_owned().into());
            let format_out = sub_matches.get_one::<Format>("OUTPUT_FORMAT").map(|x|x.to_owned().into());

            let x = IpMath::convert(ip, format_in, format_out);
            match x {
                Ok(r) => println!("{}", r),
                Err(e) => println!("{}", e)
            }
        },
        _ => println!("No args")
    }
}
