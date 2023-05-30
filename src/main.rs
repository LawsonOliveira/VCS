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
mod change_version;
mod branches;

fn main() {
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
        }
        "remove" => {
            let args: Vec<_> = std::env::args().collect();

            if args.len() < 3 {
                println!("You need to provide at least one file to remove.");
            } else {
                for arg in std::env::args().skip(2) {
                    let use_log = true;
                    remove::remove(&arg, use_log);
                }
            }
        }
        "commit" => {
            let args: Vec<_> = std::env::args().collect();

            if args.len() != 3 {
                println!("You need to provide a comment");
            } else {
                commit::create_commit(&args[2]);
            }
        },
        "delete" => {
            let args: Vec<_> = std::env::args().collect();

            if args.len() != 3 {
                println!("You need to provide the file to delete");
            } else {
                delete_commit::delete(&args[2]);
            }
        },
        "change_version" => {
            let args: Vec<_> = std::env::args().collect();

            if args.len() != 3 {
                println!("You need to provide the version to change.");
            } else {
                change_version::change_version(&args[2]);
            }
        },
        "create_branch" => {
            let args: Vec<_> = std::env::args().collect();

            if args.len() != 3 {
                println!("You need to provide the branch to create.");
            } else {
                branches::create_branch(&args[2]);
            }
        },
        "change_branch" => {
            let args: Vec<_> = std::env::args().collect();

            if args.len() != 3 {
                println!("You need to provide the branch to change.");
            } else {
                branches::change_branch(&args[2]);
            }
        },
        "delete_branch" => {
            let args: Vec<_> = std::env::args().collect();

            if args.len() != 3 {
                println!("You need to provide the branch to delete.");
            } else {
                branches::delete_branch(&args[2]);
            }
        },
        "log" => {
            print::print_log();
        },
        "init" => {
            print::print_init();
        },
        "print_commands" => {
            print::print_commands();
        },
		_ => {
			println!("Command Unknown!");
			return;
		},
	}
}