use librustbin::Client;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let _ = args.remove(0);

    if args.len() != 1 {
        println!("No argument found. Usage: ./rustbin-cli-link <link>");
        return;
    }

    let librbc = Client::new("https://bin.cyberknight777.dev".to_string());

    let paste_url = librbc
        .paste_short(args[0].to_string())
        .unwrap();

    println!("{}: {}", args[0].to_string(), paste_url.trim());
}
