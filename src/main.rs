mod lib;
use std::env;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    match &args.len() {
        3 => {
            let path = Path::new(&args[1]);
            lib::run::run(path, &args[2])
        },
        _ => eprintln!("Run ff --help")
    }
}
