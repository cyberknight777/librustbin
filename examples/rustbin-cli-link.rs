use librustbin::Client;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Usage: rustbin-cli-link <url>");
        return;
    }

    let client = Client::new("https://bin.cyberknight777.dev");
    let url = client.shorten(&args[0]).await.unwrap();
    println!("{}: {}", args[0], url.trim());
}
