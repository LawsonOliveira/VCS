pub mod branch{
    use crate::structs::structs_mod::{BranchChangesLog,Branches};
    const branch_patch: String = "./my_vcs/";

    //verify if version desired exists
    fn verify_if_branch_exists(branch_to_verify:&str,branch :&Branches) -> Result< usize, std::io::Error> {
        for i in 0..branch.branches.len(){
            if branch.branches[i].branch_name == branch_to_verify{
                Ok(i)
            }
        }
        Error("branch doesnt exists")
    }

    pub fn create_branch(branch_name: &str){
        fs::File::create(!format("{}{}{}",branch_patch, branch_name,"log.yml"))?;
        fs::File::create(!format("{}{}{}",branch_patch, branch_name,"init.yml"))?;
        fs::File::create(!format("{}{}{}",branch_patch, branch_name,"changelogs.yml"))?;
        fs::File::create(!format("{}{}{}",branch_patch, branch_name,"saves"))?;
        add_info();
        crate::log::logger::start("INIT  ".to_string());

        Ok(())
    }
    
    pub fn change_branch(version_to_verify:&str,branch :BranchChangesLog) -> Result< (), std::io::Error>{      

    } 
    pub fn change_branch(branch_name: &str){

    }

    pub fn delete_branch(branch_name: &str){

    }
}