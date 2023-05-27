pub mod print_fn
{
	// PACKAGES
	use serde_yaml::{self};
	use crate::structs::structs_mod::Log;
	use crate::structs::structs_mod::Init;



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


	fn show_log() -> Result<(), Box<dyn std::error::Error>> {

		let f = std::fs::File::open("config.yml").expect("Could not open file.");
		let log: Log = serde_yaml::from_reader(f).expect("Could not read values.");

		for i in 0..log.action.len(){
			println!("{}",log.created_date[i]);
			println!("{}",log.action[i]);
			println!("{}",log.created_time[i]);
		}
		Ok(())
	}


	pub fn print_init() {
		fn get_data() -> std::io::Result<Vec<String>>
		{
			let f = std::fs::File::open("init.yml").expect("Could not open file.");
			let init: Init = serde_yaml::from_reader(f).expect("Could not read values.");

			Ok(vec![init.created_date,init.created_time,init.current_path])
		}

		let info : Vec<String> = get_data().unwrap();

		println!("\nINFO: created date: {} - {}\n  current path: {}\n\n",
		info[0], info[1], info[2]);
	}


	//fn print_db() {
	//	let saves_base = crate::database::get::start().unwrap();
	//	let mut id : i64 = 1;

	//	for x in saves_base.into_iter() {
	//		println!("{}. {}\n   - path\n      {}\n   - saved\n      {}\n      {}\n",
	//		id, x[1], x[0], x[2], x[3]);

	//		id += 1;
	//	}
	//}


	// START POINT
	pub fn print_log() -> Result<(), Box<dyn std::error::Error>> {
		show_log()?;
		Ok(())
	}
	pub fn print_all(){
		print_init();
	}
	//pub fn print_db(){}
}