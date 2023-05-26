pub mod init_fn
{
	// PACKAGES
	use std::env;
	use std::fs;


	// START POINT
	pub fn start_vcs() {
		match fs::create_dir("my_vcs") {
			Err(why) => {
				println!("{:?} !", why.kind());
				crate::log::logger::start("INIT -> ERROR".to_string());
			}
			Ok(_) => {
				fs::File::create("./my_vcs/log.yml");
				fs::File::create("./my_vcs/init.yml");
				fs::File::create("./my_vcs/storage.yml");
				fs::create_dir("./my_vcs/saves");
				add_info();
				crate::log::logger::start("INIT  ".to_string());
			},
		}
	}


	fn add_info()
	{
		let time_date: [String; 2] = crate::time::time_funcs::start();
        //println!("{:?}", time_date);
		let cwd = env::current_dir().unwrap();
		let folder: String = String::from(cwd.to_string_lossy());
		let os_name = String::from(env::consts::OS);
		let created_date = &time_date[0];
		let created_time = &time_date[1];

		let info = format!("name: {folder}\n
                            os_name: {os_name}\n
                            created_date: {created_date}\n
                            created_time: {created_time}");

		fs::write("my_vcs/init.txt", info).expect("Unable to write file");

		println!("Initialized !");
	}



}