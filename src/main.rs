use jwalk::WalkDir;
use std::{error, io::{stderr, Read, Write}, path::Path};
use yaml_rust2::{Yaml, YamlLoader, YamlEmitter};
use clap::Parser;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Output a list of failures without saving corrections.
    #[arg(short, long)]
    check: bool,

    /// Display additional information
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.check {
        println!("ℹ️  Check mode: on!, no files will be modified.");
    }

    // recursively find yaml files below the current directory
    let dir = "./";
    let yaml_files = find_yaml_files(dir);
    
    // YamlFileResult holds the results of the yaml file validation
    struct YamlFileResult {
        file: std::path::PathBuf,
        result: Result<(), Box<dyn error::Error>>,
    }

    let mut errors_found = false;

    for file in yaml_files {
        // check if is_valid_yaml returned an error
        let file_result   = &YamlFileResult {
            file: file.clone(),
            result: is_valid_yaml(file.as_path()),
        };

        if file_result.result.is_err() {
            errors_found = true;
            writeln!(&mut stderr(), "❌ invalid yaml in {:?}: {:?}", file_result.file, file_result.result).unwrap();
        } else {
            let ok = sort_and_save_yaml_lists(file.as_path(), cli.check)
                .map_err(|e| {
                    errors_found = true;
                    writeln!(stderr(), "❌ unable to update list in {:?}: {:?}", file_result.file, e).unwrap();
                });
            if ok.is_ok() {
                // if the file was modified, print a message
                if ok.unwrap() {
                    if cli.check {
                        writeln!(stderr(), "❌ {} is unsorted", file.display()).unwrap();
                    } else {
                        println!("✅ {} sorted", file.display());
                    }
                } else {
                    if cli.verbose {
                        println!("✅ {} is already sorted", file.display());
                    }
                }
            }
        }
    }

    if errors_found {
        std::process::exit(1);
    }
}

fn find_yaml_files<P: AsRef<Path>>(path: P) -> Vec<std::path::PathBuf> {
    let mut yaml_files = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "yaml" || ext == "yml" {
                    yaml_files.push(path.to_path_buf());
                }
            }
        }
    }
    return yaml_files
}


// TODO: Remove is_valid_yaml? We wouldn't be able to sort the lists if the file is invalid
// but maybe this error is more meaningful. Needs testing.
fn is_valid_yaml<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn error::Error>> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    YamlLoader::load_from_str(&content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_is_valid_yaml_valid_file() {
        let result = is_valid_yaml("testData/test1/file-1.yaml");
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
    }

    #[test]
    fn test_is_valid_yaml_invalid_file() {
        let result = is_valid_yaml("testData/test2/test2-a/file-5-invalid.yaml");
        assert!(result.is_err(), "Expected Err, got {:?}", result);
    }

    #[test]
    fn test_find_yaml_files() {
        let yaml_files = find_yaml_files("./testData");
        assert_eq!(yaml_files.len(), 5, "Expected 5 yaml files, got {}", yaml_files.len());
        assert!(yaml_files.contains(&PathBuf::from("./testData/test2/test2-a/file-4.yaml")), "Expected ./testData/test2/test2-a/file-4.yaml to be found");
        assert!(yaml_files.contains(&PathBuf::from("./testData/test1/file-1.yaml")), "Expected ./testData/test1/file-1.yaml to be found");
    }
}

use std::fs::File;
use std::io::BufReader;

use std::error::Error;


fn sort_and_save_yaml_lists<P: AsRef<Path> + Clone>(path: P, check_flag: bool) -> Result<bool, Box<dyn Error>> {
    // Open the file and read its content
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    // Parse the YAML content
    let mut docs = YamlLoader::load_from_str(&content)?;

    // Function to recursively sort lists within the YAML structure
    fn sort_yaml(value: &mut Yaml) {
        match value {
            Yaml::Array(ref mut arr) => {
                for elem in arr.iter_mut() {
                    sort_yaml(elem);
                }
                arr.sort();
            }
            Yaml::Hash(ref mut hash) => {
                for (_, v) in hash.iter_mut() {
                    sort_yaml(v);
                }
            }
            _ => {}
        }
    }
    // Clone the original documents for comparison
    let original_docs = docs.clone();

    // Sort lists in the YAML documents
    for doc in &mut docs {
        sort_yaml(doc);
    }

    // Compare the original and sorted documents to check if they are different
    let file_modified = docs != original_docs;

    // bail early because we're in check mode
    if check_flag {
        return Ok(file_modified);
    }

    // If sorting was done, save the sorted content back to the file
    if file_modified {
        let mut sorted_yaml = String::new();
        for (i, doc) in docs.iter().enumerate() {
            let mut emitter = YamlEmitter::new(&mut sorted_yaml);
            emitter.dump(doc)?; // Dump each sorted YAML document to a String
            if i < docs.len() - 1 {
                sorted_yaml.push_str("---\n"); // Add document separator
            }
        }
        let mut file = File::create(&path)?;
        file.write_all(sorted_yaml.as_bytes())?;
    }

    Ok(file_modified)
}
