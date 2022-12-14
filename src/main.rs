use std::{fs, io};
use std::fs::{File};
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize};
use clap::Parser;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigFile {
    version: String,
    sync_files: Vec<SyncConfiguration>
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SyncConfiguration {
    file: String,
    remote: String,
    #[serde(default = "default_comment_prefix")]
    comment_prefix: String,
    #[serde(default = "default_suppress_comments")]
    suppress_comments: bool
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(default_value_t = String::from("shadows.json"), value_parser)]
    config_file: String
}

fn default_comment_prefix() -> String {
    return String::from("# ");
}
fn default_suppress_comments() -> bool {
    return false;
}

fn main() {
    let cli: Cli = Cli::parse();
    match process_config_file(&cli.config_file) {
        Ok(()) => (),
        Err(err) => {
            println!("Error: {}", err)
        }
    };
}

fn process_config_file(relative_path: &str) -> Result<(), &str> {
    let absolute_path = fs::canonicalize(relative_path).map_err(|_| "Couldn't find config file")?;
    let parent_folder = absolute_path.parent();
    if parent_folder.is_none() {
        return Err("Couldn't find parent folder");
    }
    let parent_folder = parent_folder.unwrap();

    let mut file = File::open(relative_path).map_err(|_| "Couldn't open config file")?;
    let mut data = String::new();
    file.read_to_string(&mut data).map_err(|_| "Couldn't read config file")?;
    let config: ConfigFile = serde_json::from_str(data.as_str()).map_err(|_| "Couldn't parse config file")?;
    println!("Config file version: {}", config.version);
    for sync_file in config.sync_files {
        println!("Downloading {} from {}", sync_file.file, sync_file.remote);
        match process_sync_configuration(&parent_folder, &sync_file) {
            Ok(()) => (),
            Err(err) => {
                println!("Failed to download file: {}", err);
            }
        }
    }
    return Ok(());
}

fn process_sync_configuration(parent_folder: &Path, configuration: &SyncConfiguration) -> Result<(), &'static str> {
    let path = parent_folder.join(&configuration.file);
    // let path = Path::new(&configuration.file);
    let dir = path.parent();
    if dir.is_none() {
        return Err("Invalid path");
    }
    let dir = dir.unwrap();
    fs::create_dir_all(dir).map_err(|_| "Couldn't create parent directories")?;

    let mut remote = reqwest::blocking::get(&configuration.remote).map_err(|_| "Couldn't access remote location")?;
    let mut file = File::create(path).map_err(|_| "Couldn't create or open file")?;

    if !configuration.suppress_comments {
        file.write_all(create_info(configuration).as_bytes()).map_err(|_| "Couldn't write to file")?;
    }

    io::copy(&mut remote, &mut file).map_err(|_| "Couldn't download file")?;
    return Ok(());
}

fn create_info(configuration: &SyncConfiguration) -> String {
    let mut message = String::new();
    message.push_str(&configuration.comment_prefix);
    message.push_str("This file is shadowing ");
    message.push_str(&configuration.remote);
    message.push_str("\n");

    message.push_str(&configuration.comment_prefix);
    message.push_str("All changes will be overwritten by the next sync\n\n");
    return message;
}