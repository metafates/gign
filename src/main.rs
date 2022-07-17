use clap::{ArgMatches, command};
use clap::parser::ValuesRef;

use gign::{append_to_gitignore, error, generate_gitignore, get_app_dir, get_templates, init_default_templates, pull_templates_repo, TemplateEntry};

fn main() {
    let mut cmd = command!("gign")
        .arg(
            clap::Arg::new("template")
                .help("The templates to ignore")
                .takes_value(true)
                .multiple_values(true),
        )
        .arg(
            clap::Arg::new("strict")
                .help("Do not automatically resolve unknown templates")
                .long("strict")
                .short('s')
                .takes_value(false),
        )
        .arg(
            clap::Arg::new("append")
                .help("Append to the root-level .gitignore file")
                .long("append")
                .short('a')
                .takes_value(false),
        )
        .subcommand(command!("list")
            .about("List all available templates")
            .arg(
                clap::Arg::new("prefix")
                    .help("The prefix to filter the templates")
                    .exclusive(true)
                    .takes_value(true)
                    .multiple_values(true),
            ))
        .subcommand(command!("update")
            .about("Update the default templates database"))
        .subcommand(command!("where")
            .about("Print the templates path"));

    let matches = cmd.clone().get_matches();

    if let Some(templates) = matches.get_many("template") {
        handle_template_argument(templates, matches.is_present("strict"), matches.is_present("append"));
    } else {
        match matches.subcommand() {
            Some(("list", matches)) => {
                handle_list_command(matches);
            }
            Some(("update", _)) => {
                handle_update_command();
            }
            Some(("where", _)) => {
                handle_where_command();
            }
            _ => {
                cmd.print_help().unwrap();
            }
        }
    }
}

fn handle_template_argument(templates: ValuesRef<String>, strict: bool, append: bool) -> () {
    if let Err(err) = init_default_templates() {
        error(err.to_string().as_str())
    }

    match generate_gitignore(templates, strict) {
        Ok(gitignore) => {
            if append {
                // get the current working directory
                match std::env::current_dir() {
                    Ok(cwd) => {
                        if let Err(e) = append_to_gitignore(&cwd, &gitignore) {
                            error(e.to_string().as_str());
                        }
                    }
                    Err(e) => {
                        error(e.to_string().as_str());
                    }
                }
            } else {
                println!("{}", gitignore);
            }
        }
        Err(err) => {
            error(err.to_string().as_str());
        }
    }
}

fn handle_list_command(matches: &ArgMatches) {
    if let Err(err) = init_default_templates() {
        error(err.to_string().as_str())
    }

    match get_templates() {
        Ok(available_templates) => {
            let mut entries: Vec<&TemplateEntry> = available_templates.values().collect();
            entries.sort_by(|a, b| a.prefix().cmp(b.prefix()));

            if let Some(filter_prefixes) = matches.get_many("prefix") {
                let filter_prefixes: Vec<String> = filter_prefixes.into_iter().map(|s: &String| s.to_string()).collect();
                for entry in entries {
                    if filter_prefixes.contains(&entry.prefix().to_string()) {
                        println!("{}", entry.title_colored());
                    }
                }
            } else {
                for entry in entries {
                    println!("{}", entry.title_colored());
                }
            }
        }
        Err(err) => {
            error(err.to_string().as_str())
        }
    }
}

fn handle_update_command() {
    if let Err(err) = init_default_templates() {
        error(err.to_string().as_str());
    }

    println!("Updating templates...");
    if let Err(err) = pull_templates_repo() {
        error(err.to_string().as_str())
    }

    println!("Templates updated!");
}

fn handle_where_command() {
    match get_app_dir() {
        Some(path) => {
            println!("{}", path.display());
        }
        None => {
            error("Could not find the application directory");
        }
    }
}
