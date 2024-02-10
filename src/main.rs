use std::path::Path;

use clap::Parser;

mod cli;
mod exporters;

fn main() {
    let cli = cli::Cli::parse();

    let project_path: &Path = Path::new(&cli.project_path);
    if !project_path.is_dir() {
        eprintln!("Invalid path: {:?}", project_path);
        std::process::exit(1);
    }

    match exporters::export(&cli) {
        Ok(_) => println!("Finished exporting all!"),
        Err(e) => eprintln!("Exporting failed with error {:?}", e),
    };
}
