use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use super::{ExportMode, ExportPreset};

pub struct Conf {
    pub project_path: String,
    pub godot_path: String,
    pub output_folder: String,
    pub project_name: String,
    pub project_version: String,
}

pub fn export(
    export_mode: &ExportMode,
    conf: Conf,
    export_preset: &ExportPreset,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut project_name = conf.project_name.to_owned();
    project_name.push_str("_");
    project_name.push_str(&export_preset.name);
    project_name.push_str("_");
    project_name.push_str(&conf.project_version.replace(".", "_"));
    let executable_path = PathBuf::from(&conf.output_folder)
        .join(Path::new(project_name.as_str()))
        .with_extension("exe");
    let godot_output = Command::new(conf.godot_path)
        .args([
            "--headless",
            "--path",
            conf.project_path.as_str(),
            format!("--export-{}", export_mode).as_str(),
            export_preset.name.as_str(),
            executable_path.to_str().unwrap(),
        ])
        .stderr(Stdio::inherit())
        .output()?;

    println!("Creating zip...");

    let files_to_compress = vec![
        executable_path.to_owned(),
        executable_path.with_extension("pck"),
    ];

    let options = FileOptions::default().compression_method(CompressionMethod::DEFLATE);
    let mut zip = ZipWriter::new(File::create(executable_path.with_extension("zip"))?);

    for file_path in &files_to_compress {
        let file = File::open(file_path)?;
        zip.start_file(file_path.file_name().unwrap().to_str().unwrap(), options)?;

        let mut buffer = Vec::new();
        std::io::copy(&mut file.take(u64::MAX), &mut buffer)?;

        zip.write_all(&buffer)?;

        std::fs::remove_file(file_path)?;
    }

    zip.finish()?;

    println!("Done compressing files!");

    Ok(())
}
