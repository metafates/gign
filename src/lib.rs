use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use clap::parser::ValuesRef;
use triple_accel::levenshtein_exp;
use walkdir::WalkDir;

const TEMPLATES_REPO: &str = "https://github.com/github/gitignore.git";

pub struct IgnoreTemplate {
    pub name: String,
    pub template: String,
}

pub struct TemplateEntry {
    prefix: String,
    name: String,
    template: String,
}

impl TemplateEntry {
    pub fn new(prefix: String, name: String, template: String) -> TemplateEntry {
        TemplateEntry {
            prefix,
            name,
            template,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn template(&self) -> &str {
        &self.template
    }

    pub fn title(&self) -> String {
        format!("{}:{}", self.prefix(), self.name())
    }

    pub fn to_string(&self) -> String {
        let hashes = "#".repeat(self.name().len() + 4);

        format!("
{hashes}
# {} #
{hashes}

{}



", self.name(), self.template())
    }
}


/// Get the default templates database
pub fn get_templates() -> Result<HashMap<String, TemplateEntry>, Box<dyn Error>> {
    match get_app_dir() {
        Some(path) => {
            // check if path exists
            if !path.exists() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Templates path does not exist",
                )));
            }

            // filter only files that has .gitignore extension
            let entries: Vec<TemplateEntry> = WalkDir::new(path)
                .into_iter()
                .filter_entry(|e| {
                    if e.file_type().is_file() || e.file_type().is_symlink() {
                        return e.path().extension() == Some("gitignore".as_ref());
                    }

                    !e.path().ends_with(".git") &&
                        !e.path().ends_with(".github")
                })
                .filter_map(|e| match e {
                    Ok(e) => if e.file_type().is_file() { Some(e) } else { None },
                    Err(_) => None
                }
                )
                .map(|e| {
                    let name = e.file_name().to_string_lossy().trim_end_matches(".gitignore").to_string();
                    let prefix = e.path().parent().unwrap().file_name().unwrap().to_string_lossy().to_string();
                    let template = fs::read_to_string(e.path()).unwrap();

                    if prefix == "ignore" {
                        TemplateEntry::new("".to_string(), name, template)
                    } else {
                        TemplateEntry::new(prefix.to_lowercase(), name, template)
                    }
                })
                .collect();

            let mut hash_map = HashMap::new();
            for entry in entries {
                if hash_map.contains_key(entry.title().as_str()) {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::AlreadyExists,
                        format!("Duplicate template name: {}", entry.name()),
                    )));
                }

                hash_map.insert(entry.title(), entry);
            }

            Ok(hash_map)
        }
        None => {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find the application directory",
            )))
        }
    }
}

/// Find the closest string to the given string in the given vector of strings.
/// Returns None if no suggestion is found.
pub fn find_closest<'a>(target: &str, patterns: Vec<&'a TemplateEntry>) -> Option<&'a TemplateEntry> {
    let mut closest_distance = u32::MAX;
    let mut closest_template = None;


    for pattern in patterns {
        let distance = levenshtein_exp(target.as_ref(), pattern.name().as_ref());
        if distance < closest_distance && distance < 10 {
            closest_distance = distance;
            closest_template = Some(pattern);
        }
    }

    if let Some(entry) = closest_template {
        Some(entry)
    } else {
        None
    }
}


/// Generate a gitignore file from the given template names.
pub fn generate_gitignore(mut templates: ValuesRef<String>) -> Result<String, Box<dyn Error>> {
    match get_templates() {
        Ok(available_templates) => {
            let res = templates.try_fold(
                "".to_string(),
                |acc, name| {
                    if available_templates.contains_key(name) {
                        Ok(format!("{}{}", acc, &available_templates[name].to_string()))
                    } else {
                        let available_templates: Vec<&TemplateEntry> = available_templates
                            .values()
                            .collect();

                        let closest = find_closest(name, available_templates);
                        if let Some(closest) = closest {
                            Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                format!("Template '{}' not found, did you mean '{}'?", name, closest.title()),
                            )))
                        } else {
                            Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                format!("Template '{}' not found", name),
                            )))
                        }
                    }
                },
            )?;

            // trim trailing newlines
            Ok(res.trim_matches('\n').to_string())
        }
        Err(err) => {
            return Err(err);
        }
    }
}

/// Get the application directory
pub fn get_app_dir() -> Option<PathBuf> {
    match dirs::config_dir() {
        Some(path) => Some(path.join("ignore")),
        None => None
    }
}


/// Clones the templates repository into the default directory
pub fn clone_templates_repo() -> Result<PathBuf, Box<dyn Error>> {
    match get_app_dir() {
        Some(path) => {
            let target_path = path.join("default");

            // run git clone
            let output = std::process::Command::new("git")
                .arg("clone")
                .arg(TEMPLATES_REPO)
                .arg(target_path.to_str().unwrap())
                .output()?;

            // check if clone was successful
            if !output.status.success() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to clone templates repository",
                )));
            }

            Ok(target_path)
        }
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find app directory",
        )))
    }
}

/// Update the default templates database
/// This will download the latest patterns database from the GitHub repository.
pub fn pull_templates_repo() -> Result<PathBuf, Box<dyn Error>> {
    match get_app_dir() {
        Some(path) => {
            let target_path = path.join("default");

            // run git pull
            let output = std::process::Command::new("git")
                .arg("-C")
                .arg(target_path.to_str().unwrap())
                .arg("pull")
                .arg("origin")
                .arg("main")
                .output()?;

            // check if pull was successful
            if !output.status.success() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to pull templates repository",
                )));
            }

            Ok(target_path)
        }
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find app directory",
        )))
    }
}

/// Initialize the default templates database
pub fn init_default_templates() -> Result<(), Box<dyn Error>> {
    match get_app_dir() {
        Some(path) => {
            if !path.exists() {
                fs::create_dir_all(path)?;
                println!("Cloning templates repository, first time only");
                clone_templates_repo()?;
            }

            Ok(())
        }
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find app directory",
            )));
        }
    }
}

/// Checks if the given command is available in the system path.
pub fn command_is_available(name: &str) -> bool {
    let output = std::process::Command::new("which")
        .arg(name)
        .output()
        .ok();

    output.is_some()
}

pub fn error(msg: &str) {
    eprintln!("{}", msg);
    exit(1);
}