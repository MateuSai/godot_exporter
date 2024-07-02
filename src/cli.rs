use clap::{Parser, ValueEnum};

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
    pub compress: bool,

    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ExportMode {
    Debug,
    Release,
}

impl std::fmt::Display for ExportMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportMode::Debug => write!(f, "debug"),
            ExportMode::Release => write!(f, "release"),
        }
    }
}
