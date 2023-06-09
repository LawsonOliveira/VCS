use crate::structs;
use crate::time;


/// Starts logging the specified action.
/// Returns `Ok(())` if the log was created/updated successfully, otherwise returns an `Err` with the corresponding error.
pub fn start(action: String) -> Result<(), Box<dyn std::error::Error>> {
    let time_date: [String; 2] = time::start();

    let action = action;
    let created_date = time_date[0].clone();
    let created_time = time_date[1].clone();

    let path = "my_vcs/";

    let mut log_struct: structs::structs_mod::Log = structs::StructWriter::read_struct_from_file(&format!("{}{}", path, "log.yml"))?;
    log_struct.action.push(action);
    log_struct.created_date.push(created_date);
    log_struct.created_time.push(created_time);

    // Update the struct file
    structs::StructWriter::update_struct_file(&format!("{}{}", path, "log.yml"), &log_struct)?;

    Ok(())
}

