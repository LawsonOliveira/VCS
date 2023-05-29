#![allow(warnings)]

mod init;
mod print;
mod time;
mod commit;
mod add;
mod remove;
mod structs;
mod log;
mod delete_commit;
mod versions;
use std::fs;

pub fn VCS_is_initialized(dir_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(dir_path) {
        metadata.is_dir()
    } else {
        false
    }
}

fn main() {
    const path : &str = "my_vcs";
    // Check if the user provided more than 1 argument
    {
        let args: Vec<_> = std::env::args().collect();

        if args.len() == 1 {
            println!("You should enter arguments to use the program.\n");
            print::print_commands();
            return;
        }
    }

    let first_arg = std::env::args().nth(1).expect("no function given");

    match first_arg.as_ref() {
        "init" => init::start_vcs(),
        "add" => {
            if VCS_is_initialized(path){
                let args: Vec<_> = std::env::args().collect();
                if args.len() < 3 {
                    println!("You need to provide at least one file to add.");
                } else {
                    for arg in std::env::args().skip(2) {
                        let file_exists = std::path::Path::new(&arg).exists();

                        if file_exists {
                            add::add_to_version_control(&arg);
                        } else {
                            println!("File does not exist: {}", arg);
                        }
                    }
                }
            }else{
                println!("You must initialize first!")
            }

        }
        "remove" => {
            if VCS_is_initialized(path){

                let args: Vec<_> = std::env::args().collect();

                if args.len() < 3 {
                    println!("You need to provide at least one file to remove.");
                } else {
                    for arg in std::env::args().skip(2) {
                        let use_log = true;
                        remove::remove(&arg, use_log);
                    }
                }
            }else{
                println!("You must initialize first!")
            }
        }
        "version" => {
            if VCS_is_initialized(path){

                let args: Vec<_> = std::env::args().collect();

                if args.len() < 3 {
                    println!("You need to provide the version");
                } else {
                    versions::version_fn::change_version(&args[2]);
                }
            }else{
                println!("You must initialize first!")
            }
        },

        "commit" => {
            if VCS_is_initialized(path){

                let args: Vec<_> = std::env::args().collect();

                if args.len() != 3 {
                    println!("You need to provide a comment for the commit.");
                } else {
                    commit::commit(&args[2]);
                }
            }else{
                println!("You must initialize first!")
            }
        },
        "delete" => {
            let args: Vec<_> = std::env::args().collect();

            if args.len() != 3 {
                println!("You need to provide a comment for the commit.");
            } else {
                delete_commit::delete(&args[2]);
            }
        },
		"print_log" => {
            if VCS_is_initialized(path){
                print::print_log();
            }else{
                println!("You must initialize first!")
            }
        },
        "print_init" => {
            if VCS_is_initialized(path){
                print::print_init();
            }else{
                println!("You must initialize first!")
            }
        },

        "commands" =>{
            if VCS_is_initialized(path){
                print::print_commands();
            }else{
                println!("You must initialize first!")
            }
        },

		&_ => todo!(),
		//"select" => {
		//	let scd_arg = std::env::args().nth(2).expect("save to select wasn't given");
		//	select_fn::start(scd_arg);
		//},
		//"CreateBranch" => {			
		//	let scd_arg = std::env::args().nth(2).expect("branch to create wasn't given");
			//select_fn::start(scd_arg);
		//},
		//"ChangeBranch" => {			
		//	let scd_arg = std::env::args().nth(2).expect("branch to change wasn't given");
			//select_fn::start(scd_arg);
		//},
		//"ChangeVersion" => {	
		//	let scd_arg = std::env::args().nth(2).expect("version to change wasn't given");
			//select_fn::start(scd_arg);
		//},
		//"exit" => return,
		//_ => {
			//println!("Command Unknown!");
			//return;
		//},
	}
}