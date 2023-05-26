#![allow(warnings)]

// PACKAGES
use std::io;

// MODULES
mod init;
use crate::init::init_fn;


mod print;
use crate::print::print_fn;

mod time;
use crate::time::time_funcs;

mod commit;
use crate::commit::commit_fn;

mod add;
use crate::add::add_fn;

mod remove;
use crate::remove::remove_fn;

mod structs;
mod log;




fn main() {
	//Check if the user provided more than 1 argument
	{
		let args: Vec<_> = std::env::args().collect();

		if args.len() == 1 {		
			println!("You should enter arguments to use the program.\n");

			print_fn::print_commands();

			return;
		}
	}	

	let first_arg = std::env::args().nth(1).expect("no function given");

	match first_arg.as_ref() {
		"init" => init_fn::start_vcs(),
		"add" => {
			let args: Vec<_> = std::env::args().collect();

			if args.len() < 3 {
				println!("you nee to provide at least one file to add");
			}
			else{
				for i in 3..args.len(){
					let arg = std::env::args().nth(i).expect("missing the arg");
					let fl_exist = std::path::Path::new(&arg).exists();
		
					if fl_exist == true { add_fn::add(&arg); }
					else { println!("not exist"); }

				}
			}
		},
		"remove" => {
			let args: Vec<_> = std::env::args().collect();

			if args.len() < 3 {
				println!("you nee to provide at least one file to remove");
			}
			else{
				for i in 3..args.len(){
					let arg = std::env::args().nth(i).expect("missing the arg");
					let fl_exist = std::path::Path::new(&arg).exists();
		
					if fl_exist == true { add_fn::add(&arg); }
					else { println!("not exist"); }

				}
			}
		}, &_ => todo!(),
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
		//"print" => print_fn::start(0),
		//"log" => print_fn::start(1),
		//"info" => print_fn::read_yaml(),
		//"cmd" => print_fn::print_commands(),
		//"exit" => return,
		//_ => {
			//println!("Command Unknown!");
			//return;
		//},
	}
}