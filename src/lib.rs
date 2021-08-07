pub mod run {
    use std::path::Path;

    use colored::*;
    pub fn run(dir: &Path, required_ext: &String) {
        let files = std::fs::read_dir(dir).unwrap();
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
