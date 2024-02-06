use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use super::ExportPreset;

pub struct Conf {
    pub project_path: String,
    pub godot_path: String,
    pub output_folder: String,
    pub project_name: String,
}

pub fn export(conf: Conf, export_preset: ExportPreset) -> Result<(), Box<dyn std::error::Error>> {
    let executable_path = PathBuf::from(&conf.output_folder)
        .join(Path::new(conf.project_name.as_str()))
        .with_extension("exe");
    let godot_command = Command::new(conf.godot_path)
        .args([
            "--headless",
            "--path",
            conf.project_path.as_str(),
            "--export-release",
            export_preset.name.as_str(),
            executable_path.to_str().unwrap(),
        ])
        .spawn();

    godot_command?.wait()?;

    println!("Compressing files...");

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
