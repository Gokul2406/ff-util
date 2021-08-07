pub mod run {
    pub fn run() {
        let files = std::fs::read_dir(".").unwrap();
        let required_ext = "toml";
        for file in files {
            if let Ok(f) = &file {
                if let Some(f_as_str) = f.file_name().to_str() {
                    let file_ext: Vec<&str> = f_as_str.split(".").collect();
                    for ext in &file_ext {
                        if ext.contains(required_ext) {
                            for matched_file in &file {
                                println!("{:?}", matched_file.file_name()); 
                            }
                        }
                    }
                }
            }
        }
    }
}
