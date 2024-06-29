use std::{
    fs::File,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use super::{ExportMode, ExportPreset};

#[derive(Debug)]
pub struct Conf {
    pub project_path: String,
    pub godot_path: String,
    pub output_folder: String,
    pub project_name: String,
    pub project_version: String,
    pub project_icon: PathBuf,
    pub compress: bool,
}

pub fn export(
    export_mode: &ExportMode,
    conf: Conf,
    export_preset: &ExportPreset,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("conf: {:?}", conf);

    let executable_path = PathBuf::from(&conf.output_folder).join(Path::new(
        conf.project_name
            .to_lowercase()
            .replace(" ", "_")
            .replace("/", "_")
            .as_str(),
    ));
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

    let files = vec![
        executable_path.to_owned(),
        executable_path.with_extension("pck"),
        executable_path.with_extension("desktop"),
        PathBuf::from(&conf.output_folder).join("install.sh"),
    ];

    let container_path = PathBuf::from(conf.output_folder).join(format!(
        "{}_{}_{}",
        executable_path
            .file_name()
            .expect("Linux executable does not have file_name")
            .to_str()
            .unwrap(),
        export_preset
            .name
            .to_lowercase()
            .replace(" ", "_")
            .replace("/", "_"),
        conf.project_version.replace(".", "_")
    ));

    if conf.compress {
        println!("Creating tar.gz...");

        let tar_path = container_path.with_extension("tar.gz");
        println!("tar path: {}", &tar_path.to_str().unwrap());
        let tar_gz = File::create(tar_path)?;
        let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
        let mut tar = tar::Builder::new(enc);
        for file_path in files {
            println!("Adding {} to tar.gz...", file_path.to_str().unwrap());
            tar.append_file(file_path.file_name().unwrap(), &mut File::open(&file_path)?)?;
            std::fs::remove_file(&file_path)?;
            println!("Added {} to tar.gz", file_path.to_str().unwrap());
        }

        tar.append_file(
            executable_path.with_extension("png").file_name().unwrap(),
            &mut File::open(conf.project_icon)?,
        )?;

        println!("tar.gz created!");
    } else {
        println!("Adding files inside foder");

        let folder_path = container_path;
        std::fs::create_dir(folder_path.to_owned())?;
        println!("Folder created");

        for file_path in &files {
            std::fs::rename(
                file_path,
                folder_path.to_owned().join(file_path.file_name().unwrap()),
            )?;
        }
    }

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
