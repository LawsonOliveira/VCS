/// Provides functions for initializing the VCS.
use std::env;
use std::fs;
use std::io;

use crate::structs;
use crate::time;

/// Starts the VCS initialization process.
pub fn start_vcs() {
    match fs::create_dir("my_vcs") {
        Err(why) => {
            println!("{:?} !", why.kind());
            crate::log::start("INIT -> ERROR".to_string());
        }
        Ok(_) => {
            //fs::File::create("my_vcs/log.yml");
            //fs::File::create("my_vcs/init.yml");
            fs::create_dir("my_vcs/saves");

            if let Err(err) = structs::StructWriter::write_blank_structs_to_files(&structs::StructWriter) {
                // Handle the error
                if let Some(io_error) = err.downcast_ref::<io::Error>() {
                    // Handle IO error
                    eprintln!("IO error: {}", io_error);
                } else if let Some(yaml_error) = err.downcast_ref::<serde_yaml::Error>() {
                    // Handle YAML error
                    eprintln!("YAML error: {}", yaml_error);
                } else {
                    // Handle other errors
                    eprintln!("Unknown error: {}", err);
                }
            }

            add_info();
            crate::log::start(format!("init"));
        }
    }
}

/// Adds information to the VCS initialization file.
///
/// # Errors
///
/// Returns an error if there is an issue reading or updating the struct file.
fn add_info() -> Result<(), Box<dyn std::error::Error>> {
    let time_date: [String; 2] = time::start();
    let cwd = env::current_dir().unwrap().to_string_lossy().to_string();
    let os_name = String::from(env::consts::OS);
    let created_date = time_date[0].clone();
    let created_time = time_date[1].clone();
    let path = "my_vcs/";

    let mut init_struct: structs::structs_mod::Init = structs::StructWriter::read_struct_from_file(&format!("{}{}", path, "init.yml"))?;
    init_struct.created_date = created_date;
    init_struct.created_time = created_time;
    init_struct.current_path = cwd;

    // Update the struct file
    structs::StructWriter::update_struct_file(&format!("{}{}", path, "init.yml"), &init_struct)?;

    println!("Initialized!");

    Ok(())
}
