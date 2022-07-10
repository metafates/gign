use std::process::exit;

use clap::command;

use ignore::generate_gitignore;

fn main() {
    let cmd = command!("ignore")
        .arg(
            clap::Arg::with_name("template")
                .short('t')
                .long("template")
                .help("The templates to ignore")
                .exclusive(true)
                .takes_value(true)
                .multiple_values(true),
        )
        .arg(
            clap::Arg::with_name("update")
                .short('u')
                .long("update")
                .help("Update the default templates database")
                .takes_value(false)
                .exclusive(true),
        );

    let matches = cmd.get_matches();

    if let Some(templates) = matches.get_many("template") {
        match generate_gitignore(templates) {
            Ok(gitignore) => {
                println!("{}", gitignore);
                exit(0);
            }
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        }
    } else if matches.is_present("update") {
        println!("Updating patterns database");
    } else {
        println!("No pattern provided");
    }
}
