pub mod run {
    use std::path::Path;

    use colored::*;
    pub fn run(dir: &Path, required_ext: &String) {
        // We use the read_dir method to read the directory given by the user and it is stored in a
        // variable for future use
        let files = std::fs::read_dir(dir).unwrap();

        // Now we loop over the files vector and checks if the file contains the extension or the
        // filename provided by the user
        for file in files {
            if let Ok(f) = &file {
                if let Some(f_as_str) = f.file_name().to_str() {
                    let file_ext: Vec<&str> = f_as_str.split(".").collect();
                    for ext in &file_ext {
                        if ext.contains(required_ext) {
                            for matched_file in &file {
                                if let Some(matched) = matched_file.file_name().to_str() {
                                    println!("{}", matched.bold().yellow());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
