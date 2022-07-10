use clap::command;
use clap::parser::ValuesRef;

use ignore::{error, generate_gitignore, get_templates, init_default_templates, TemplateEntry};

fn main() {
    let cmd = command!("ignore")
        .about("Generate a gitignore file from the given template names.")
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
            clap::Arg::with_name("list")
                .short('l')
                .long("list")
                .help("List all available templates")
                .takes_value(false)
                .exclusive(true),
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
        handle_template_argument(templates);
    } else if matches.is_present("list") {
        handle_list_argument();
    } else if matches.is_present("update") {
        hand_update_argument()
    } else {
        println!("No pattern provided");
    }
}

fn handle_template_argument(templates: ValuesRef<String>) -> () {
    if let Err(err) = init_default_templates() {
        error(err.to_string().as_str())
    }

    match generate_gitignore(templates) {
        Ok(gitignore) => {
            println!("{}", gitignore);
        }
        Err(err) => {
            error(err.to_string().as_str());
        }
    }
}

fn handle_list_argument() {
    if let Err(err) = init_default_templates() {
        error(err.to_string().as_str());
    }

    match get_templates() {
        Ok(available_templates) => {
            let mut entries: Vec<&TemplateEntry> = available_templates.values().collect();
            entries.sort_by(|a, b| a.prefix().cmp(b.prefix()));

            for entry in entries {
                println!("{}", entry.title_colored());
            }
        }
        Err(err) => {
            error(err.to_string().as_str());
        }
    }
}

fn hand_update_argument() {
    println!("Updating patterns database");
}