pub mod time_funcs
{
    use chrono::{Local, DateTime};
    fn get_date(local: DateTime<Local>) -> String
    {
        let date = local.format("%d/%m/%y").to_string();
        return date
    }

    fn get_time(local: DateTime<Local>) -> String
    {
        let time = local.format("%H:%M:%S").to_string();
        return time
    }

    pub fn start() -> [String; 2] {
        let local: DateTime<Local> = Local::now();
        let res: [String; 2] = [get_date(local), get_time(local)];
        return res;
    }
}