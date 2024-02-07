use std::path::Path;

use clap::Parser;

mod exporters;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_hint = clap::ValueHint::DirPath, default_value = ".")]
    project_path: String,

    #[arg(short, long, value_hint = clap::ValueHint::ExecutablePath)]
    godot_path: String,

    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    output_folder: String,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let project_path: &Path = Path::new(&args.project_path);
    if !project_path.is_dir() {
        eprintln!("Invalid path: {:?}", project_path);
        std::process::exit(1);
    }

    match exporters::export(args.project_path, args.godot_path, args.output_folder) {
        Ok(_) => println!("Finished exporting!"),
        Err(e) => eprintln!("Exporting failed with error {:?}", e),
    };
}
