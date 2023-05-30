// PACKAGES
use serde_yaml;
use crate::structs::structs_mod::{Log, Init};
use crate::structs::StructWriter;
/// Prints available commands.
pub fn print_commands() {
    println!("Available commands:");
    println!("init: Initialize the version control system");
    println!("add <file>: Add a file to version control");
    println!("remove <file>: Remove a file from version control");
    println!("commit <comment>: Create a commit with a comment");
    println!("delete <file>: Delete a file from version control");
    println!("change_version <version>: Change the version of a file");
    println!("create_branch <branch>: Create a new branch");
    println!("change_branch <branch>: Change to a different branch");
    println!("delete_branch <branch>: Delete a branch");
    println!("log: Print the commit log");
    println!("init: Print the initialization details");
    println!("print_commands: Print available commands");
}


/// Shows the log entries.
///
/// # Errors
///
/// Returns an error if there is an issue opening or reading the log file.
fn show_log() -> Result<(), Box<dyn std::error::Error>> {

    let log: Log= StructWriter::read_struct_from_file("my_vcs/log.yml").unwrap();

    for i in 0..log.action.len() {
        println!("Action: {} - Date: {} - Created time: {}",log.action[i],log.created_date[i],log.created_time[i]);
    }

    Ok(())
}


/// Prints project initialization information.
pub fn print_init() {
    fn get_data() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let f = std::fs::File::open("init.yml")?;
        let init: Result<Init, serde_yaml::Error> = serde_yaml::from_reader(f);

        match init {
            Ok(init) => Ok(vec![init.created_date, init.created_time, init.current_path]),
            Err(err) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string(),
            ))),
        }
    }

    let info: Vec<String> = match get_data() {
        Ok(data) => data,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    println!(
        "\nINFO: created date: {} - {}\n  current path: {}\n\n",
        info[0], info[1], info[2]
    );
}

/// Prints the log entries.
///
/// # Errors
///
/// Returns an error if there is an issue showing the log.
pub fn print_log() -> Result<(), Box<dyn std::error::Error>> {
    show_log()?;
    Ok(())
}

