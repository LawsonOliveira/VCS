pub mod commit_fn
{
	// PACKAGES
	use std::fs;
    use diffy;
    

    fn gen_diff(original_text_path: &str, modified_text_path: &str, save_path: &str) {
        // Read the contents of the original text file
        let original_text = match fs::read_to_string(original_text_path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("Error reading original text file: {}", error);
                return;
            }
        };
    
        // Read the contents of the modified text file
        let modified_text = match fs::read_to_string(modified_text_path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("Error reading modified text file: {}", error);
                return;
            }
        };
    
        // Create a patch based on the differences between the original and modified texts
        let patch = diffy::create_patch(&original_text, &modified_text);
    
        // Create a new file for writing the patch
        let file = match fs::File::create(save_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Error creating file: {}", error);
                return;
            }
        };
    
        // Write the patch into the file
        if let Err(error) = diffy::PatchFormatter::new().write_patch_into(&patch, &file) {
            eprintln!("Error writing diff to file: {}", error);
            return;
        }
    }


    fn apply_patch(original_text_path: &str, diff_patch_path: &str, save_path: &str) {
        // Read the contents of the original text file
        let original_text = match fs::read_to_string(original_text_path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("Error reading original file: {}", error);
                return;
            }
        };
    
        // Read the contents of the patch file
        let patch_text = match fs::read_to_string(diff_patch_path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("Error reading patch file: {}", error);
                return;
            }
        };
    
        // Parse the patch text into a Patch object
        let patch = match diffy::Patch::from_str(&patch_text) {
            Ok(patch) => patch,
            Err(error) => {
                eprintln!("Error parsing patch: {}", error);
                return;
            }
        };
    
        // Apply the patch to the original text
        let new_text_version = match diffy::apply(&original_text, &patch) {
            Ok(new_text) => new_text,
            Err(error) => {
                eprintln!("Error applying patch: {}", error);
                return;
            }
        };
    
        // Write the new version of the text to a file
        if let Err(error) = fs::write(save_path, new_text_version) {
            eprintln!("Error writing new version: {}", error);
            return;
        }
    }

    
    //pub fn commit(){}
    
    /*pub fn test(){
        let original_text_path: &str;
        let modified_text_path: &str;
        let save_path: &str;
        let version_3: &str;
        original_text_path = "./my_vcs/saves/text1.yaml";
        modified_text_path = "./my_vcs/saves/text2.yaml";
        save_path = "./my_vcs/saves/diffs.yaml";
        version_3 = "./my_vcs/saves/textv3.yaml";
        gen_diff(&original_text_path, &modified_text_path, &save_path);
        apply_patch(&original_text_path, &save_path, &version_3);
        assert_eq!(fs::read_to_string(&modified_text_path).unwrap(), fs::read_to_string(&version_3).unwrap());

    }*/
    
}





