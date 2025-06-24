use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
    week: u8,
}

fn main() {
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}", args.command, args.week);
}
