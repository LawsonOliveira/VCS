// PACKAGES
use serde_yaml;
use crate::structs::structs_mod::{Log,Init,FileChangeLog};
use crate::structs::StructWriter;
/// Prints available commands.
pub fn print_commands() {
    println!("Commands:\n");
    println!("my_vcs init\n  command to initialize the project\n");
    println!("my_vcs add src/main.js save_name\n  command to save the file.\n");
    println!("my_vcs delete save_name\n  command to delete the save.\n");
    println!("my_vcs select save_name\n  command to insert saved content into a file.\n");
    println!("my_vcs commit \"comment to be save\"\n  command to commit the save\n");
    println!("my_vcs CreateBranch NameBranchToCreate \n  command to create a new branch\n");
    println!("my_vcs ChangeBranch NameBranchToChange\n  command to change the current branch to another\n");
    println!("my_vcs ChangeVersion NameVersionToChange\n  command to browse between the version of the code\n");
    println!("my_vcs print\n  command to display all saves and info about project.\n");
    println!("my_vcs info\n  command to view information about the initialized project.\n");
    println!("my_vcs cmd\n  command to display all commands.\n");
    println!("my_vcs log\n  command to view logs.\n");
}

/// Prints project initialization information.
pub fn print_init() {
    let  info : Init = StructWriter::read_struct_from_file("my_vcs/init.yml").unwrap();

    println!(
        "\nINFO: created date: {} - {}\n  current path: {}\n\n",
        info.created_date, info.created_time, info.current_path
    );
}

/// Prints the log entries.
///
/// # Errors
///
/// Returns an error if there is an issue showing the log.
/// 
/// 

/// Prints all information including project initialization and log entries.
pub fn print_log(){
    let log: Log = StructWriter::read_struct_from_file("my_vcs/log.yml").unwrap();

    for i in 0..log.action.len() {
        if log.action.contains(&"commit".to_string()){
            println!("{}", log.created_date[i]);
            println!("{}", log.action[i]);
            println!("{}", log.created_time[i]);
        }else{
            println!("{}", log.created_date[i]);
            println!("{}", log.action[i]);
            println!("{}", log.created_time[i]);
        }
    }
}

