use clap::Parser;

use crate::exporters::ExportMode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[arg(short, long, value_hint = clap::ValueHint::DirPath, default_value = ".")]
    pub project_path: String,

    #[arg(short, long, value_hint = clap::ValueHint::ExecutablePath)]
    pub godot_path: String,

    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub output_folder: String,

    #[arg(long)]
    pub presets: Vec<String>,

    #[arg(long, default_value = "debug")]
    pub export_mode: ExportMode,

    #[arg(short, long)]
    pub verbose: bool,
}
