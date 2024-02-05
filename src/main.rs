use std::path::Path;

use clap::Parser;

use crate::exporters::get_version;

mod exporters;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_hint = clap::ValueHint::DirPath, default_value = ".")]
    project_path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let project_path: &Path = &args.project_path;
    if !project_path.is_dir() {
        eprintln!("Invalid path: {:?}", project_path);
        std::process::exit(1);
    }

    exporters::export();

    println!(
        "Project version: {}",
        get_version(args.project_path.clone())
    );

    println!("Hello, world! {:?}", args.project_path);
}
