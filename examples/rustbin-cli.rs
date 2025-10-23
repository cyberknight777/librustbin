use librustbin::Client;
use std::ffi::OsStr;
use std::{env, fs, path::Path};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let _ = args.remove(0);

    let librbc = Client::new("https://bin.cyberknight777.dev".to_string());

    for arg in args {
        let path = Path::new(&arg);
        let ext = path.extension().and_then(OsStr::to_str).unwrap_or("txt");

        if !path.exists() {
            println!("{}: Not found", &arg);
            continue;
        }

        let paste_url = librbc
            .paste_highlight(fs::read_to_string(&path).unwrap())
            .unwrap();
        println!("{}: {}.{}", arg, paste_url.trim(), ext);
    }
}
