pub mod print_fn
{
	// PACKAGES
	use std::fs;
	use std::fs::File;
	use std::io::{prelude::*, BufReader};


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


	fn show_log() -> std::io::Result<()> {
		let file = fs::File::open("my_vcs/log.txt")?;
		let reader = BufReader::new(file);
		for line in reader.lines() { println!("{}", line?); }

		Ok(())
	}


	pub fn read_yaml() {
		fn cut_data(x: String) -> String
		{
			let mut elem_start_point : i64 = 0;
			let mut s = String::new();

			for i in x.chars() {
				if i == ':' { elem_start_point += 1; }
				if elem_start_point >= 1 { s.push(i); }
			}

			s.remove(0);
			s.remove(0);

			return s;
		}

		fn get_data() -> std::io::Result<Vec<String>>
		{
			let file = File::open("my_vcs/init.txt")?;
			let reader = BufReader::new(file);
			let mut info_base = Vec::new();

			for line in reader.lines() { info_base.push(cut_data(line.unwrap())); }

			Ok(info_base)
		}

		let info : Vec<String> = get_data().unwrap();

		println!("\nINFO\n  os: {}\n  created date: {} - {}\n  current path: {}\n\n",
		info[1], info[3], info[2], info[0]);
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
	pub fn start(x: i64) {
		if x == 1 { show_log(); }
		else {
			crate::log::logger::start("PRINT ".to_string());
			read_yaml();
			//print_db();
		}
	}
}