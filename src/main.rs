mod lib;
use std::env;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]); 
    let extension = &args[2];
    lib::run::run(&path, &extension);
}
