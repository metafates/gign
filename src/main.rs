use clap::command;
use clap::parser::ValuesRef;

use ignore::{
    error,
    generate_gitignore,
    get_app_dir,
    get_templates,
    init_default_templates,
    pull_templates_repo,
    TemplateEntry,
};

fn main() {
    let mut cmd = command!("ignore")
        .arg(
            clap::Arg::new("template")
                .help("The templates to ignore")
                .exclusive(true)
                .takes_value(true)
                .multiple_values(true),
        )
        .subcommand(command!("list")
            .about("List all available templates"))
        .subcommand(command!("update")
            .about("Update the default templates database"))
        .subcommand(command!("where")
            .about("Print the templates path"));

    let matches = cmd.clone().get_matches();

    if let Some(templates) = matches.get_many("template") {
        handle_template_argument(templates);
    } else {
        match matches.subcommand() {
            Some(("list", _)) => {
                handle_list_command();
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

fn handle_list_command() {
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

fn handle_update_command() {
    if let Err(err) = init_default_templates() {
        error(err.to_string().as_str());
    }

    println!("Updating templates...");
    if let Err(err) = pull_templates_repo() {
        error(err.to_string().as_str());
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