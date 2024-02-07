use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Command,
};

use super::ExportPreset;

#[derive(Debug)]
pub struct Conf {
    pub project_path: String,
    pub godot_path: String,
    pub output_folder: String,
    pub project_name: String,
}

pub fn export(conf: Conf, export_preset: ExportPreset) -> Result<(), Box<dyn std::error::Error>> {
    println!("conf: {:?}", conf);

    let executable_path = PathBuf::from(&conf.output_folder).join(Path::new(
        conf.project_name.to_lowercase().replace(" ", "_").as_str(),
    ));
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

    println!("Creating .desktop file...");

    std::fs::write(
        executable_path.with_extension("desktop"),
        get_desktop_file_text(&conf.project_name),
    )?;

    println!(".desktop file created!");

    println!("Creating install script...");

    std::fs::write(
        PathBuf::from(&conf.output_folder).join("install.sh"),
        get_install_script_text(&conf.project_name),
    )?;

    println!("install script created!");

    println!("Creating tar.gz...");

    let files_to_compress = vec![
        executable_path.to_owned(),
        executable_path.with_extension("pck"),
        executable_path.with_extension("desktop"),
        PathBuf::from(&conf.output_folder).join("install.sh"),
    ];

    let tar_gz = File::create(executable_path.with_extension("tar.gz"))?;
    let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);
    for file_path in files_to_compress {
        tar.append_file(file_path.file_name().unwrap(), &mut File::open(&file_path)?)?;
        std::fs::remove_file(file_path)?;
    }

    println!("tar.gz created!");

    Ok(())
}

fn get_desktop_file_text(project_name: &str) -> String {
    format!(
        "[Desktop Entry]
Type=Application
Name={}
Exec=sh -c /usr/local/bin/{}
Categories=Game;",
        project_name,
        project_name.to_lowercase().replace(" ", "_")
    )
}

fn get_install_script_text(project_name: &str) -> String {
    let project_name_without_spaces_and_lowercase = project_name.to_lowercase().replace(" ", "_");
    format!(
        "#!/bin/sh

BINARY_DIRECTORY=/usr/local/bin/

cp -f {} $BINARY_DIRECTORY/{}
cp -f {}.pck $BINARY_DIRECTORY/{}.pck
cp -f {}.png /usr/share/pixmaps/{}.png
cp -f {}.desktop /usr/share/applications/{}.desktop
chmod u+x /usr/share/applications/{}.desktop",
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
    )
}
