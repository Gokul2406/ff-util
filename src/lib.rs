pub mod run {
    pub fn run() {
       let files = std::fs::read_dir(".").unwrap();
       for file in files {
           if let Ok(f) = file {
               if let Some(f_as_str) = f.file_name().to_str() {
                println!("{}", f_as_str);
               }
               }
           }
       }
    }
