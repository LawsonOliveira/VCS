#![allow(warnings)]

// PACKAGES
use std::io;

// MODULES
mod init;
pub use crate::init::init_fn;

//mod add_file;
//pub use crate::add_file::add_fn;

//mod delete;
//pub use crate::delete::delete_fn;

//mod select;
//pub use crate::select::select_fn;

mod print;
pub use crate::print::print_fn;

mod log;

mod time;
pub use crate::time::time_funcs;
//mod database;
// MODULES


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
		"init" => init_fn::start_vcs(), &_ => todo!(),
		//"add" => {
		//	let args: Vec<_> = std::env::args().collect();

		//	if args.len() == 3 {
		//		let scd_arg = std::env::args().nth(2).expect("missing the argument 2");
		//		let thd_arg = std::env::args().nth(3).expect("missing the argument 3");

		//		let fl_exist = std::path::Path::new(&scd_arg).exists();

		//		if fl_exist == true { add_fn::start(&scd_arg, &thd_arg); }
		//		else { println!("not exist"); }
		//	}
		//	else
		//	{ println!("you didnt provide the right number of arguments"); }
		//},
		//"delete" => {
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
