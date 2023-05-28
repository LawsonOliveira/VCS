#![allow(warnings)]

mod init;
mod print;
mod time;
mod commit;
mod add;
mod remove;
mod structs;
mod log;

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
                println!("You need to provide a comment for the commit.");
            } else {
                commit::commit(&args[2]);
            }
        },
		//"print" => print::start(0),
		//"log" => print::start(1),
		&_ => todo!(),
		//	let scd_arg = std::env::args().nth(2).expect("save to delete wasn't given");
		//	delete_fn::start(scd_arg);
		//},
		//"select" => {
		//	let scd_arg = std::env::args().nth(2).expect("save to select wasn't given");
		//	select_fn::start(scd_arg);
		//},
		//"commit" => {
		//	let scd_arg = std::env::args().nth(2).expect("commit wasn't given");
			//select_fn::start(scd_arg);
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
		//"info" => print::read_yaml(),
		//"cmd" => print::print_commands(),
		//"exit" => return,
		//_ => {
			//println!("Command Unknown!");
			//return;
		//},
	}
}