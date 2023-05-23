use chrono::prelude::*;
pub mod time_fn
{
    let local: DateTime<Local> = Local::now();

    
    fn get_date() -> String
    {
        let date = local.format("%d/%m/%y").to_string();
        
        date
    }
    
    fn get_time() -> String
	{
        let time = local.format("%H:%M:%S").to_string();

		time
	}
	pub fn start() -> [String; 2] {
		let res: [String; 2] = [get_date(), get_time()];
		return res;
	}
}