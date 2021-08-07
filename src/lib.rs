pub mod run {
    pub fn run() {
       let files = std::fs::read_dir(".").unwrap();
       for file in files {
           if let Ok(f) = file {
                println!("{:?}", f.path());
           }
       }
    }
}
