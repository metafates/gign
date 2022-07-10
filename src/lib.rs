use std::collections::HashMap;
use std::error::Error;

use clap::parser::ValuesRef;
use triple_accel::levenshtein_exp;

pub struct IgnoreTemplate {
    pub name: String,
    pub template: String,
}

/// Update the default templates database
/// This will download the latest patterns database from the GitHub repository.
pub fn update() -> Result<(), Box<dyn Error>> {
    Ok(())
}

/// Get the default templates database
pub fn get_templates() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let hash_map = HashMap::from([
        ("rust".to_string(), "*.rs".to_string()),
        ("java".to_string(), "*.java".to_string()),
        ("js".to_string(), "*.js".to_string()),
        ("ts".to_string(), "*.ts".to_string()),
    ]);
    Ok(hash_map)
}

/// Find the closest string to the given string in the given vector of strings.
/// Returns None if no suggestion is found.
pub fn find_closest(target: &str, patterns: Vec<&String>) -> Option<String> {
    let mut closest_distance = 100;
    let mut closest_template_name = None;


    for pattern in patterns {
        let distance = levenshtein_exp(target.as_ref(), pattern.as_ref());
        if distance < closest_distance {
            closest_distance = distance;
            closest_template_name = Some(pattern);
        }
    }

    if let Some(name) = closest_template_name {
        Some(name.to_string())
    } else {
        None
    }
}


/// Generate a gitignore file from the given template names.
pub fn generate_gitignore(templates: ValuesRef<String>) -> Result<String, String> {
    let available_templates = get_templates().unwrap();
    let mut templates_to_ignore: ValuesRef<String> = templates;

    let mut gitignore = String::new();

    let res: Result<(), String> = templates_to_ignore.try_for_each(|template| {
        if available_templates.contains_key(template) {
            gitignore.push_str(
                &format!(
                    "{}\n\n",
                    available_templates[template]
                )
            );

            Ok(())
        } else {
            let available_templates_names: Vec<&String> = available_templates.keys().collect();
            let suggestion = find_closest(
                template,
                available_templates_names,
            );

            let mut err = format!("Unknown template \"{}\"", template);
            if let Some(suggestion) = suggestion {
                err.push_str(&format!("\nDid you mean \"{}\"?", suggestion));
            }

            Err(err)
        }
    });

    if res.is_err() {
        Err(res.err().unwrap())
    } else {
        Ok(gitignore)
    }
}
