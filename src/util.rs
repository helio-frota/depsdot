use std::path::PathBuf;
use std::{fs, io};

use toml::Value;

/// This function gets all the Cargo.toml files from a directory, and returns a
/// vector with all the paths of the Cargo.toml files.
pub fn tomls(dir: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut files = Vec::new();
    // Gets all the entries from 'current|argument' dir.
    let entries = fs::read_dir(dir)?;

    for e in entries {
        let entry = e?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str.contains("target") {
                        continue;
                    }
                }
            }

            // Take a look at sub dir
            let temp = tomls(&path.to_string_lossy())?;
            // .rs found ? Then add to the vector.
            files.extend(temp);
        } else if let Some(file_name) = path.file_name() {
            // not a dir and Cargo.toml
            if file_name == "Cargo.toml" {
                // Then add to the vector.
                files.push(path);
            }
        }
    }

    Ok(files)
}

pub fn parse_tomls(cargos: Vec<PathBuf>) -> Vec<Value> {
    cargos
        .into_iter()
        .filter_map(|path| {
            let content = fs::read_to_string(path).ok()?;
            let cargo_toml: Value = toml::from_str(&content).ok()?;
            Some(cargo_toml)
        })
        .collect()
}

pub fn grouped_dependencies(cargos: Vec<Value>) {
    for c in cargos {
        if let Some(package) = c.get("package") {
            if let Some(name) = package.get("name") {
                println!("name: {}", name);
                if let Some(deps) = c.get("dependencies") {
                    println!("deps: {}", deps);
                }
            } else {
                println!("No package name found");
            }
        }
    }
}
