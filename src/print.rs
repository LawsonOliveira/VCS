// PACKAGES
use serde_yaml;
use crate::structs::structs_mod::{Log, Init};

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

/// Shows the log entries.
///
/// # Errors
///
/// Returns an error if there is an issue opening or reading the log file.
fn show_log() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("config.yml")?;
    let log: Log = serde_yaml::from_reader(f)?;

    for i in 0..log.action.len() {
        println!("{}", log.created_date[i]);
        println!("{}", log.action[i]);
        println!("{}", log.created_time[i]);
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

/// Prints all information including project initialization and log entries.
pub fn print_all() {
    print_init();
    // print_db();
}

// fn print_db() {
//     let saves_base = crate::database::get::start().unwrap();
//     let mut id: i64 = 1;

//     for x in saves_base.into_iter() {
//         println!(
//             "{}. {}\n   - path\n      {}\n   - saved\n      {}\n      {}\n",
//             id, x[1], x[0], x[2], x[3]
//         );

//         id += 1;
//     }
// }

