use librustbin::{Client, PasteOptions};
use std::ffi::OsStr;
use std::{env, fs, path::Path};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: rustbin-cli <file> [file...]");
        return;
    }

    let client = Client::new("https://bin.cyberknight777.dev");

    for arg in &args {
        let path = Path::new(arg);

        if !path.exists() {
            eprintln!("{arg}: not found");
            continue;
        }

        let content = fs::read_to_string(path).unwrap();
        let filename = path.file_name().and_then(OsStr::to_str).map(String::from);

        let options = PasteOptions {
            filename,
            ..Default::default()
        };

        let url = client.paste(&content, &options).await.unwrap();
        println!("{arg}: {}", url.trim());
    }
}
