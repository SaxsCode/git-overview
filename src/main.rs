use clap::Parser;
use gitlab::Gitlab;
use gitlab::api::{self, Query, projects};
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

// 1. Retrieve project by user
// 2. Loop through them with git log cmd
// 3. Parse output
// 4. Generate txt file
//
#[derive(Parser)]
struct Cli {
    command: String,
    week: u8,
}

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
}

fn main() {
    dotenv().ok();
    // let args = Cli::parse();

    let gitlab_host = std::env::var("GITLAB_HOST").expect("No GITLAB_HOST found in env");
    let gitlab_token = std::env::var("GITLAB_TOKEN").expect("No GITLAB_TOKEN found in env");
    let client = Gitlab::new(gitlab_host, gitlab_token);

    println!("client: {:?}", client);
}
