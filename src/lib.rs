use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit};

use clap::parser::ValuesRef;
use colored::Colorize;
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
    template: Option<String>,
    path: PathBuf,
}

impl TemplateEntry {
    pub fn new(prefix: String, name: String, path: PathBuf) -> TemplateEntry {
        TemplateEntry {
            prefix,
            name,
            template: None,
            path,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn with_template(&self) -> Result<Self, Box<dyn Error>> {
        let template = fs::read_to_string(&self.path)?;
        Ok(TemplateEntry {
            prefix: self.prefix.clone(),
            name: self.name.clone(),
            template: Some(template),
            path: self.path.clone(),
        })
    }

    pub fn template(&self) -> Option<&String> {
        match &self.template {
            Some(template) => Some(template),
            None => None,
        }
    }

    pub fn title(&self) -> String {
        if self.prefix.is_empty() {
            self.name.clone()
        } else {
            format!("{}:{}", self.prefix(), self.name())
        }
    }

    pub fn title_colored(&self) -> String {
        if self.prefix.is_empty() {
            self.name.clone()
        } else {
            format!("{}{}{}", self.prefix().green(), ":".magenta(), self.name())
        }
    }

    pub fn to_string(&self) -> String {
        let hashes = "#".repeat(self.name().len() + 4);
        let name = self.name();
        let template = self.template().unwrap();

        format!("
{hashes}
# {} #
{hashes}

{}



", name, template)
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

            let entries: Vec<TemplateEntry> = WalkDir::new(path)
                .into_iter()
                .filter_entry(|e| {
                    // filter only files that has .gitignore extension
                    if e.file_type().is_file() || e.file_type().is_symlink() {
                        return e.path().extension() == Some("gitignore".as_ref());
                    }

                    // ignore non-related directories
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

                    if prefix == "ignore" {
                        TemplateEntry::new("".to_string(), name, e.path().to_path_buf())
                    } else {
                        TemplateEntry::new(prefix.to_lowercase(), name, e.path().to_path_buf())
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
                        let template = available_templates.get(name).unwrap();
                        Ok(format!("{}{}", acc, template.with_template().unwrap().to_string()))
                    } else {
                        let available_templates: Vec<&TemplateEntry> = available_templates
                            .values()
                            .collect();

                        let closest = find_closest(name, available_templates);
                        if let Some(closest) = closest {
                            Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                format!("Template '{}' not found, did you mean '{}'?", name, closest.title_colored()),
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
            // check if git is installed
            if !command_is_available("git") {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Git is not installed",
                )));
            }

            let target_path = path.join("default");

            // run git clone
            let output = Command::new("git")
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
            let output = Command::new("git")
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
    let output = Command::new("which")
        .arg(name)
        .output()
        .ok();

    output.is_some()
}

pub fn error(msg: &str) {
    eprintln!("{}", msg);
    exit(1);
}