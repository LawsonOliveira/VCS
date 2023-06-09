use crate::structs;
use crate::log;
use crate::create_branch::verify_if_branch_exists;


/// Change the current branch to the one with the given name.
/// Returns `Ok(())` if the branch was changed successfully, otherwise returns an `Err` with the corresponding error.
pub fn change_branch(branch_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";

    // Verify if the branch exists
    if !verify_if_branch_exists(branch_name)? {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Branch does not exist")));
    }

    let mut repository: structs::structs_mod::Repository =
        structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml")).unwrap();

    repository.current_branch = branch_name.to_string();
    structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;
    log::start(format!("change_branch {}", &branch_name));
    println!("Now you are working in the branch '{}'", branch_name);
    Ok(())
}
