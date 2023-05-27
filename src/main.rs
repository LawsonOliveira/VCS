#![allow(warnings)]

// PACKAGES
use std::io;

// MODULES
mod init;
use crate::init::init_fn;

mod test;
use test::test_fn;
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
				for arg in std::env::args().skip(2) {
					let fl_exist = std::path::Path::new(&arg).exists();
				
					if fl_exist {
						add_fn::add(&arg);
					} else {
						println!("File does not exist");
					}
				}
			}
		},
		"remove" => {
			let args: Vec<_> = std::env::args().collect();

			if args.len() < 3 {
				println!("you nee to provide at least one file to remove");
			}
			else{
				for arg in std::env::args().skip(2) {
					//let fl_exist = std::path::Path::new(&arg).exists();
					remove_fn::remove(&arg);

					//if fl_exist {
					//	remove_fn::remove(&arg);
					//} else {
					//	println!("File does not exist");
					//}
				}
			}
		}, 
		"commit" =>{
			let args: Vec<_> = std::env::args().collect();

			if args.len() < 3 || args.len()>3 {
				println!("you need to provide at least one file to remove");
			}
			else{commit_fn::commit(&args[2].to_string());}
		},
		"test" =>test_fn::my_test(),
		//"print" => print_fn::start(0),
		//"log" => print_fn::start(1),
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
		//"info" => print_fn::read_yaml(),
		//"cmd" => print_fn::print_commands(),
		//"exit" => return,
		//_ => {
			//println!("Command Unknown!");
			//return;
		//},
	}
}