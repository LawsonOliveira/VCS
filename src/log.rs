pub mod logger
{
	use std::fs;
	use std::io::Write;
    use serde_yaml;

    use crate::structs::structs_mod;
    use crate::time::time_funcs;

    pub fn start(action: String) {
        let time_date: [String; 2] = time_funcs::start();

        let mut log_to_save = structs_mod::Log {
            action: vec![action.clone()],
            created_date: vec![time_date[0].clone()],
            created_time: vec![time_date[1].clone()],
        };

        let file_path = "./my_vcs/log.yml";

        let f = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path);

        match f {
            Ok(mut file) => {
                match serde_yaml::to_writer(&file, &log_to_save) {
                    Ok(_) => println!("Log saved"),
                    Err(_) => eprintln!("Couldn't save the log"),
                }
            }
            Err(error) => eprintln!("Error opening file: {}", error),
        }
    }
}

