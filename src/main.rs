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
        _ => eprintln!("
File finder needs some arguments here is how you should run it\n
ff [DIRECTORY] [FILE/FILE EXTENSION]\n
Example \n
ff ~/wallpapers wallpaper.png

                       ")
    }
}
